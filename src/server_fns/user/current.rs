use super::*;

#[server]
pub async fn get_current_user(token: Option<String>) -> Result<Option<Usr>, ServerFnError<AuthError>>
{
	use ls_service::{utils::jwt::decode_jwt, UserQuery};

	use crate::app::state::AppState;

	println!("get_current_user called with token: {:?}", token);
	if token.is_none()
	{
		return Ok(None);
	}
	else
	{
		let token = token.unwrap();

		// decode token and get user id and email from it
		let data = decode_jwt(token).expect("Could not decode JWT");

		let state = leptos_actix::extract().await;
		let state: AppState = state.unwrap();
		let conn = state.conn;

		// find user by id
		let user = UserQuery::find_user_by_id(&conn, data.claims.id).await;
		if user.is_err()
		{
			return Err(ServerFnError::WrappedServerError(AuthError::NoUserFound));
		}
		let user = user.unwrap();
		let user = user.unwrap();

		if user.id != data.claims.id || user.email != data.claims.email
		{
			return Err(ServerFnError::WrappedServerError(AuthError::NoUserFound));
		}

		let current = Usr { id: user.id, username: user.username, email: user.email };
		Ok(Some(current))
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Usr
{
	pub id:       i32,
	pub username: String,
	pub email:    String
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
			_ => Err("Invalid AuthError".to_string())
		}
	}
}
