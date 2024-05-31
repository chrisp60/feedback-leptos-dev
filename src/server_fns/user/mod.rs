pub mod login;
pub mod register;

use super::*;

#[server(CheckAuth, "/dash")]
pub async fn check_auth() -> Result<AuthType, ServerFnError<AuthError>>
{
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");
	match access_token.get()
	{
		Some(token) =>
		{
			info!("Access token found: {:?}", token);
			Ok(AuthType::Authorized(token))
		}
		None =>
		{
			info!("Access token not found");
			Ok(AuthType::UnAuthorized)
		}
	}
}

use anyhow::Result;
use leptos::SignalGet;
use leptos_use::{use_cookie, utils::FromToStringCodec};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AuthType
{
	Authorized(String),
	UnAuthorized
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AuthError
{
	UnableToCheckAuth
}

impl std::fmt::Display for AuthError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			AuthError::UnableToCheckAuth => write!(f, "UnableToCheckAuth")
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
			"UnableToCheckAuth" => Ok(AuthError::UnableToCheckAuth),
			_ => Err("Invalid AuthError".to_string())
		}
	}
}
