use super::*;

#[server(UserLogin, "/login")]
pub async fn login(identity: String, password: String) -> Result<(), ServerFnError<LoginError>>
{
	use actix_web::{cookie::{time::Duration, Cookie},
	                http::{header, header::HeaderValue}};
	use leptos_actix::ResponseOptions;
	use ls_service::UserQuery;

	use crate::app::state::AppState;

	println!("Login server function called for {} with {}", identity, password);

	let state = leptos_actix::extract().await;
	let state: AppState = state.unwrap();
	let conn = state.conn;

	let reply = UserQuery::authenticate_user(&conn, &identity, &password).await;

	if reply.is_err()
	{
		println!("Login error: {:?}", reply);
		return Err(ServerFnError::WrappedServerError(LoginError::NoUserFound));
	}
	let reply = reply.unwrap();

	// println!("Login Response: {:?}", reply.clone());
	let response = expect_context::<ResponseOptions>();

	let cookie = Cookie::build("leptos_access_token", reply.clone()).path("/").http_only(true).max_age(Duration::minutes(10)).finish();

	if let Ok(cookie) = HeaderValue::from_str(cookie.to_string().as_str())
	{
		response.insert_header(header::SET_COOKIE, cookie);
	}

	// let token_context = use_context::<AccessToken>();
	// if token_context.is_none()
	// {
	// 	println!("Token context in login is none, providing new context");
	// 	let access_token = access_token_provider();
	// 	provide_context(access_token);
	// }
	// else
	// {
	// 	println!("Token context in login is some");
	// 	let token_set = token_context.unwrap().set_access_token;
	// 	token_set(Some(cookie.to_string()));
	// }

	leptos_actix::redirect("/dashboard");

	println!("Login successful, redirecting to dashboard");

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
