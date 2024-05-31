// use anyhow::Result;
// use leptos::SignalGet;
// use leptos_use::{use_cookie, utils::FromToStringCodec};
// use log::info;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub enum AuthType
// {
// 	Authorized(String),
// 	UnAuthorized
// }

// async fn check_auth() -> Result<AuthType>
// {
// 	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");
// 	match access_token.get()
// 	{
// 		Some(token) =>
// 		{
// 			info!("Access token found: {:?}", token);
// 			Ok(AuthType::Authorized(token))
// 		}
// 		None =>
// 		{
// 			info!("Access token not found");
// 			Ok(AuthType::UnAuthorized)
// 		}
// 	}
// }
