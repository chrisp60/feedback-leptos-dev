use super::*;

#[server(UserLogin, "/login")]
pub async fn login(_identity: String, _password: String) -> Result<(), ServerFnError<LoginError>>
{
	use actix_web::{cookie::{time::Duration, Cookie},
	                http::{header, header::HeaderValue}};
	use leptos_actix::ResponseOptions;

	use crate::{app::{jwt::encode_jwt, state::AppState},
	            server_fns::user::current::Usr};

	let state = leptos_actix::extract().await;
	let state: AppState = state.unwrap();
	let _conn = state.conn;

	// pretending to use database connection to authenticate user
	let usr = Usr { id: 1, username: "test".to_string(), email: "test@gmail.com".to_string() };
	let token = encode_jwt(usr.email.clone(), usr.id).expect("Could not encode JWT");

	let reply = move || -> anyhow::Result<String> { Ok(token) };
	let r2 = reply.clone();

	if reply().is_err()
	{
		return Err(ServerFnError::WrappedServerError(LoginError::NoUserFound));
	}
	else
	{
		let reply = r2().unwrap();

		let response = expect_context::<ResponseOptions>();

		let cookie = Cookie::build("leptos_access_token", reply.clone()).path("/").http_only(true).max_age(Duration::minutes(10)).finish();

		if let Ok(cookie) = HeaderValue::from_str(cookie.to_string().as_str())
		{
			response.insert_header(header::SET_COOKIE, cookie);
		}

		leptos_actix::redirect("/dashboard")
	};

	Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LoginError
{
	NoUserFound
}

impl std::fmt::Display for LoginError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			LoginError::NoUserFound => write!(f, "NoUserFound")
		}
	}
}

impl FromStr for LoginError
{
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		match s
		{
			"NoUserFound" => Ok(LoginError::NoUserFound),
			_ => Err("Invalid LoginError".to_string())
		}
	}
}
