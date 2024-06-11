use super::*;

#[server(UserAuth, "")]
pub async fn auth(id: i32) -> Result<(), ServerFnError<AuthError>>
{
	use actix_web::{cookie::{time::Duration, Cookie},
	                http::{header, header::HeaderValue}};
	use leptos_actix::ResponseOptions;
	use ls_service::UserQuery;

	use crate::app::state::AppState;

	println!("Auth server function called for {}", id);

	let state = leptos_actix::extract().await;
	let state: AppState = state.unwrap();
	let conn = state.conn;

	let reply = UserQuery::find_user_by_id(&conn, id).await;

	if reply.is_err()
	{
		println!("Auth error: {:?}", reply);
		return Err(ServerFnError::WrappedServerError(AuthError::NoUserFound));
	}
	let reply = reply.unwrap();

	println!("Auth Response: {:?}", reply.clone());
	// let response = expect_context::<ResponseOptions>();

	// let cookie = Cookie::build("leptos_access_token", reply.clone()).path("/").http_only(true).max_age(Duration::minutes(10)).finish();

	// if let Ok(cookie) = HeaderValue::from_str(cookie.to_string().as_str())
	// {
	// 	response.insert_header(header::SET_COOKIE, cookie);
	// }

	Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AuthError
{
	NoUserFound
}

impl std::fmt::Display for AuthError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			AuthError::NoUserFound => write!(f, "NoUserFound")
		}
	}
}

impl FromStr for AuthError
{
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		match s
		{
			"NoUserFound" => Ok(AuthError::NoUserFound),
			_ => Err("Invalid LoginError".to_string())
		}
	}
}
