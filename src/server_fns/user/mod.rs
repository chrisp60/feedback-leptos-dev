pub mod login;
pub mod logout;
pub mod register;

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

// #[server(CheckAuthToken, "/api/dash")]
// pub async fn check_auth_token(token: Option<String>) -> Result<(), ServerFnError<AuthError>>
// {
// 	use ls_service::utils::jwt::decode_jwt;

// 	println!("Checking token");

// 	match token
// 	{
// 		None =>
// 		{
// 			println!("No token provided");
// 			return Err(ServerFnError::WrappedServerError(AuthError::MissingToken));
// 		}
// 		Some(token) =>
// 		{
// 			let result = decode_jwt(token);
// 			println!("Result = {:?}", result);

// 			if result.is_err()
// 			{
// 				println!("Error decoding access token: {:?}", result);
// 				return Err(ServerFnError::WrappedServerError(AuthError::DecodeTokenError));
// 			}
// 			else
// 			{
// 				println!("Access token decoded: {:?}", result);

// 				let rst = result.unwrap().claims;
// 				println!("Access Token = Expiry {:?}", rst.exp);

// 				let now = chrono::Utc::now().timestamp() as usize;
// 				println!("Now = {:?}", now);

// 				let dif = rst.exp - now;

// 				println!("Difference = {:?}", dif);

// 				if rst.exp < now
// 				{
// 					println!("Access token expired");
// 					return Err(ServerFnError::WrappedServerError(AuthError::ExpiredToken));
// 				}
// 				else if rst.email == ""
// 				{
// 					println!("Access token invalid");
// 					return Err(ServerFnError::WrappedServerError(AuthError::InvalidToken));
// 				}
// 				else
// 				{
// 					println!("Access token valid");
// 					Ok(())
// 				}
// 			}
// 		}
// 	}
// }

// use anyhow::Result;
// use leptos::SignalGet;
// use leptos_use::{use_cookie, utils::FromToStringCodec};
// use log::info;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub enum AuthError
// {
// 	DecodeTokenError,
// 	ExpiredToken,
// 	InvalidToken,
// 	MissingToken
// }

// impl std::fmt::Display for AuthError
// {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
// 	{
// 		match self
// 		{
// 			AuthError::DecodeTokenError => write!(f, "TokenDecodeError"),
// 			AuthError::ExpiredToken => write!(f, "ExpiredToken"),
// 			AuthError::InvalidToken => write!(f, "InvalidToken"),
// 			AuthError::MissingToken => write!(f, "MissingToken")
// 		}
// 	}
// }

// impl FromStr for AuthError
// {
// 	type Err = String;

// 	fn from_str(s: &str) -> Result<Self, Self::Err>
// 	{
// 		match s
// 		{
// 			"InvalidToken" => Ok(AuthError::InvalidToken),
// 			"TokenDecodeError" => Ok(AuthError::DecodeTokenError),
// 			"ExpiredToken" => Ok(AuthError::ExpiredToken),
// 			"MissingToken" => Ok(AuthError::MissingToken),
// 			_ => Err("Invalid AuthError".to_string())
// 		}
// 	}
// }
