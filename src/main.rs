use actix::*;
use actix_files as fs;
use actix_web::{error, http, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Instant;

use std::fmt;

use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};

use oauth2::basic::{BasicClient, BasicErrorResponse, BasicTokenResponse, BasicTokenType};
use oauth2::reqwest::http_client;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl, Client};
use uuid::Uuid;

mod actor_models;
mod messages;

use actor_models::{ChatServer, WsChatSession};

use serde::Deserialize;
use std::process::exit;
use actix_files::NamedFile;

mod session_models;

/// AuthID has two methods to identify a user
/// `uuid` is used as OAuth2 login,
/// `secret_key` is used as Admin
/// otherwise it's a anonymous user.
#[derive(Deserialize, Debug)]
struct Auth {
    secret_key: Option<String>,
    uuid: Option<String>,
}

/// Entry point for our route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
    web::Query(auth): web::Query<Auth>,
    server_sessions: web::Data<session_models::SessionMap>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", req);

    let identity = if auth.secret_key.is_some() {
        // will be first login as higher priority.
        // TODO: implement check algorithm.
        actor_models::Identity::Admin("Admin".to_owned())
    } else {
        // get users name
        let name = auth
            .uuid
            .map(|id| {
                server_sessions
                    .get_user_info(&id)
                    .map(|ui| ui.preferred_username)
            })
            .flatten();
        match name {
            None => actor_models::Identity::Anonymous,
            Some(name)=> actor_models::Identity::User(name),
        }
    };


    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "Test".into(),
            identity,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

async fn identity(id: Identity) -> String {
    // access request identity
    if let Some(id) = id.identity() {
        format!("Welcome! {}", id)
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[derive(Deserialize, Debug)]
struct AuthCode {
    //    session_state: String,
    state: CsrfToken,
    code: String,
}

fn err2internal_err<T>(err: T) -> Error
where
    T: fmt::Debug + fmt::Display + 'static,
{
    error::ErrorInternalServerError(err)
}

async fn oidc_redirected(
    req: HttpRequest,
    web::Query(auth_code): web::Query<AuthCode>,
    id: Identity,
    server_session: web::Data<session_models::SessionMap>,
    client: web::Data<Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    // let uuid = session.get::<String>("uuid").unwrap().unwrap();
    if let Some(uuid) = id.identity() {
        // better err handling
        let mut session_map = server_session.user_data.lock().unwrap();
        let user_data = session_map
            .remove(&uuid)
            .ok_or_else (|| error::ErrorBadRequest("invalid flow: user_data missing."))?;

        let (pkce_verifier, csrf_token) = user_data
            .credentials
            .ok_or_else(|| error::ErrorBadRequest("invalid flow: hadn't access /login/."))?;

        if csrf_token.secret() !=  auth_code.state.secret() {
            return Err(error::ErrorUnauthorized("wrong csrf token."))
        }

        // Now you can trade it for an access token.
        let token_result = client
            .exchange_code(AuthorizationCode::new(auth_code.code))
            // Set the PKCE code verifier.
            .set_pkce_verifier(pkce_verifier)
            .request(http_client)
            .map_err(err2internal_err)?;

        // let ui = get_userinfo(token_result.access_token().secret())?;
        let ui = token_result
            .access_token()
            .secret()
            .parse::<session_models::UserInfo>()?;

        session_map.insert(
            uuid.to_owned(),
            session_models::UserData {
                credentials: None,
                user_info: Some(ui),
            },
        );

        Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .header(http::header::SET_COOKIE, format!("uuid={}; Path=/", uuid))
            .finish())
    } else {
        Err(error::ErrorBadRequest("invalid flow: you don't have session."))
    }
}



/// first, we redirect to the identity provider to login
async fn login(
    req: HttpRequest,
    id: Identity,
    server_session: web::Data<session_models::SessionMap>,
    client: web::Data<Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", req);


    if let Some(id) = id.identity() {
        // has already login, redirect to index page.
        if server_session.is_login(&id) {
            return Ok(HttpResponse::Found()
                .header(http::header::LOCATION, "/")
                .finish());
        }
    }



    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    let uuid = format!("{}", Uuid::new_v4());

    // session.set("uuid", uuid.to_owned());

    id.remember(uuid.to_owned());

    let mut session_map = server_session.user_data.lock().unwrap();
    session_map.insert(
        uuid,
        session_models::UserData {
            credentials: Some((pkce_verifier,csrf_token)),
            user_info: None,
        },
    );

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, auth_url.as_str())
        .finish())
}

/// default port
fn default_port() -> u32 {
    80
}

/// default address
fn default_address() -> String {
    "0.0.0.0".to_owned()
}

/// default config struct
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct Config {
    client_id:String,
    client_secret: String,
    redirect_url: String,
    auth_url: String,
    token_url: String,
    #[serde(default = "default_address")]
    address : String,
    #[serde(default = "default_port")]
    port: u32,
}
/// then
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().unwrap();

    let config = match envy::from_env::<Config>() {
        Ok(c) => {println!("{:#?}",c);
        c}
        Err(err) => {
            println!("env error: {:#?}", err);
            exit(1);
        }
    };
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    // BasicTokenResponse<EmptyExtraTokenFields, BasicTokenType>, BasicTokenType
    let client =
        BasicClient::new(
            ClientId::new(config.client_id),
            Some(ClientSecret::new(config.client_secret)),
            AuthUrl::new(config.auth_url).map_err(err2internal_err).unwrap(),
            Some(TokenUrl::new(config.token_url).map_err(err2internal_err).unwrap()),
        )
            // Set the URL the user will be redirected to after the authorization process.
            .set_redirect_url(RedirectUrl::new(config.redirect_url).map_err(err2internal_err).unwrap());

    let client =web::Data::new(client);


    env_logger::init();

    // Start chat server actor
    let server = ChatServer::default().start();
    let session_map = web::Data::new(session_models::SessionMap::default());

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .app_data(session_map.clone())
            .app_data(client.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .data(server.clone())
            // redirect to index.html
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/ws/").to(chat_route))
            .service(web::resource("/identity/").to(identity))
            .service(web::resource("/login/").route(web::get().to(login)))
            .service(web::resource("/login/redirected/").route(web::get().to(oidc_redirected)))
            .service(fs::Files::new("/static/", "static/"))
    })
    .bind(format!("{}:{}", config.address,config.port))?
    .run()
    .await
}

async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

