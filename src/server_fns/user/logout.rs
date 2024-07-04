use super::*;

#[server(UserLogout, "/logout")]
pub async fn logout_user() -> Result<(), ServerFnError<LogOutError>> {
    use actix_web::{
        cookie::{
            time::{Duration, OffsetDateTime},
            Cookie,
        },
        http::{header, header::HeaderValue},
    };
    use leptos_actix::ResponseOptions;

    // (chrisp60): see note @ src/main.rs
    tracing::info!("Logging out");
    let response = expect_context::<ResponseOptions>();
    let cookie = Cookie::build("leptos_access_token", "".to_string())
        .path("/")
        .http_only(true)
        .expires(OffsetDateTime::now_utc() - Duration::days(1))
        .finish();

    if let Ok(cookie) = HeaderValue::from_str(cookie.to_string().as_str()) {
        response.insert_header(header::SET_COOKIE, cookie);
    }

    leptos_actix::redirect("/login");

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutError {
    NoCookie,
}

impl std::fmt::Display for LogOutError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogOutError::NoCookie => write!(f, "NoCookie"),
        }
    }
}

impl FromStr for LogOutError {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoCookie" => Ok(LogOutError::NoCookie),
            _ => Err("Invalid LogOutError".to_string()),
        }
    }
}
