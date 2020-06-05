//! Implement server logic, unrelated to actor.

use oauth2::{CsrfToken, PkceCodeVerifier};
use std::collections::HashMap;

use crate::err2internal_err;
use failure::_core::str::FromStr;
use jsonwebtoken::dangerous_unsafe_decode;
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize, Clone, Debug)]
pub struct UserInfo {
    email: String,
    name: String,
    pub preferred_username: String,
}

impl FromStr for UserInfo {
    type Err = actix_web::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dangerous_unsafe_decode::<UserInfo>(s)
            .map(|td| td.claims)
            .map_err(err2internal_err)
    }
}

pub struct UserData {
    pub credentials: Option<
        (PkceCodeVerifier, CsrfToken), // Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType>,
    >,
    pub user_info: Option<UserInfo>,
}

pub struct SessionMap {
    pub user_data: Mutex<HashMap<String, UserData>>,
}

impl SessionMap {
    // fn retrieve_credentials(
    //     &mut self,
    //     id: &str,
    // ) -> Option<(
    //     PkceCodeVerifier,
    //     Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType>,
    // )> {
    //     unimplemented!()
    // }

    pub fn is_login(&self, id: &str) -> bool {
        let session_map = self.user_data.lock().unwrap();
        if let Some(user_data) = session_map.get(id) {
            user_data.user_info.is_some()
        } else {
            false
        }
    }

    pub fn get_user_info(&self, id: &str) -> Option<UserInfo> {
        let session_map = self.user_data.lock().unwrap();
        session_map
            .get(id)
            .map(|user_data| user_data.user_info.clone())
            .flatten()
    }
}

impl Default for SessionMap {
    fn default() -> Self {
        SessionMap {
            user_data: Mutex::new(HashMap::new()),
        }
    }
}
