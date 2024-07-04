use crate::{app::jwt::Claims, error::Error};

use super::*;

#[server]
pub async fn get_current_user(token: Option<String>) -> Result<Option<Usr>, ServerFnError> {
    use crate::{app::state::AppState, server_fns::user::logout::logout_user};

    let Some(token) = token else {
        return Ok(None);
    };

    let Ok(claims) = Claims::decode(token) else {
        logout_user().await.map_err(ServerFnError::new)?;
        return Ok(None);
    };

    let state = leptos_actix::extract::<AppState>()
        .await
        .map_err(Error::custom)?;

    let _conn = state.conn;

    // find user by id
    let user = Usr {
        id: 1,
        username: "test".to_string(),
        email: "test@gmail.com".to_string(),
    };

    if (claims.email != user.email) || (claims.id != user.id as i64) {
        return Ok(None);
    }

    let current = Usr {
        id: user.id,
        username: user.username,
        email: user.email,
    };

    Ok(Some(current))
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Usr {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl Default for Usr {
    fn default() -> Self {
        Self {
            id: -1,
            username: "".to_string(),
            email: "".to_string(),
        }
    }
}

impl FromStr for Usr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let usr: Usr = serde_json::from_str(s).map_err(|e| e.to_string())?;
        Ok(usr)
    }
}

impl std::fmt::Display for Usr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
