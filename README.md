# Danmaku-Server

a danmaku server based on [Actix-web](https://github.com/actix/actix-web),
compatible with [DPlayer](https://github.com/MoePlayer/DPlayer).

# Feature

- [x] Authenticate with Oauth2. (Anonymous can only receive danmaku)
- [x] Multiple danmaku room
- [x] Configure from `.env`

# Configure

```Bash
# Required
# this config is required by OAuth2.
CLIENT_ID=OAuth2_id
# this config is required by OAuth2.
CLIENT_SECRET=secret
# this config is required by OAuth2.
REDIRECT_URL=https://danmaku.test.com
# this config is required by OAuth2.
TOKEN_URL=https://danmaku.test.com
# this config is required by OAuth2.
AUTH_URL=IP

# Optional, will be default if not provided
ADDRESS=0.0.0.0
PORT=80

```
