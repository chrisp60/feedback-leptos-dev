use super::*;

#[server]
pub async fn get_current_user(token: Option<String>) -> Result<Option<Usr>, ServerFnError<AuthError>>
{
	use crate::{app::{jwt::decode_jwt, state::AppState},
	            server_fns::user::logout::logout_user};

	if token.is_none()
	{
		Ok(None)
	}
	else
	{
		let token = token.unwrap();

		// decode token and get user id and email from it
		let data = decode_jwt(token);
		if data.is_err()
		{
			print!("Error decoding token - logging out");
			let _ = logout_user().await;
			return Ok(None);
		}
		let data = data.unwrap();

		let state = leptos_actix::extract().await;

		if state.is_err()
		{
			return Err(ServerFnError::WrappedServerError(AuthError::NoUserFound));
		}

		let state: AppState = state.unwrap();
		let _conn = state.conn;

		// find user by id
		let user = anyhow::Result::<Option<Usr>>::Ok(Some(Usr { id: 1, username: "test".to_string(), email: "test@gmail.com".to_string() }));
		if user.is_err()
		{
			println!("Error finding user");
			return Err(ServerFnError::WrappedServerError(AuthError::NoUserFound));
		}
		let user = user.unwrap();
		let user = user.unwrap();

		if user.id != data.claims.id || user.email != data.claims.email
		{
			println!("User not found {:?}, {:?}, {:?},{:?}", user.id, data.claims.id, user.email, data.claims.email);
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

impl Default for Usr
{
	fn default() -> Self
	{
		Self { id: -1, username: "".to_string(), email: "".to_string() }
	}
}

impl FromStr for Usr
{
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		let usr: Usr = serde_json::from_str(s).map_err(|e| e.to_string())?;
		Ok(usr)
	}
}

impl std::fmt::Display for Usr
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{}", serde_json::to_string(self).unwrap())
	}
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
