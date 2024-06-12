#[cfg(feature = "ssr")]
pub mod state;

pub(crate) mod pages;
use leptos::*;

#[component]
pub fn App() -> impl IntoView
{
	use leptos_meta::*;
	use leptos_router::*;
	use leptos_use::{use_cookie, utils::FromToStringCodec};

	use crate::app::pages::{dash::dashboard::{DashOptions, DashboardPage},
	                        home::HomePage,
	                        test::TestPage,
	                        user::{login::LoginPage, register::RegisterPage},
	                        NotFound};
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// let auth_action = create_server_action::<UserAuth>();

	// let err = Signal::derive(move || {
	// 	(auth_action.value())().map_or("".to_owned(), |result| {
	// 		                       match result
	// 		                       {
	// 			                       Ok(a) =>
	// 		                           {
	// 			                           return "Success:- User authenticated".to_string();
	// 		                           }
	// 		                           Err(err) =>
	// 		                           {
	// 			                           let e = format!("{:?}", err);

	// 			                           if e.contains("NoUserFound")
	// 			                           {
	// 				                           return "Error:- Unable to find a user with those credentials. Please check and try again!".to_string();
	// 			                           }
	// 			                           else
	// 			                           {
	// 				                           return "Error:- Unknown error occurred.".to_string();
	// 			                           }
	// 		                           }
	// 		                       }
	// 	                       })
	// });

	// let (user_update_ready, trigger_user) = create_signal(false);

	let usr = create_blocking_resource(move || access_token.get(), get_current_user);

	// create_effect(move |_| usr.and_then(|u: &Option<Usr>| println!("user resource is = {:?}", u)));

	//create_effect(move |_| trigger_user.set(true));

	let username = Signal::derive(move || usr.get().map(|data| data.unwrap().map_or("".to_string(), |usr| usr.username.clone())));

	view! {
		<Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

		// sets the document title
		<Title text="Welcome to Leptos"/>

		// content for this welcome page
		<Router>
			<main>
				<Routes>
					<Route path="" view=HomePage/>
					<Route path="register" view=RegisterPage/>
					<Route
						path="/login"
						view=move || {
							view! {
								<Suspense fallback=move || {
									"Loading...."
								}>
									{move || {
										usr.get()
											.map(|data| {
												let rr = data.unwrap();
												view! {
													<Show
														when=move || rr.is_none()
														fallback=move || view! { <Redirect path="/dashboard"/> }
													>
														<LoginPage/>
													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					/>

					<Route
						path="/dashboard"
						view=move || {
							view! {
								<Suspense fallback=move || {
									"Loading...."
								}>
									{move || {
										usr.get()
											.map(|data| {
												let rr = data.unwrap();
												view! {
													<Show
														when=move || rr.is_some()
														fallback=move || view! { <Redirect path="/login"/> }
													>
														<DashboardPage/>
														<br/>
														{"rr is some - should be showing dashboard page"}
														<br/>
														{username}
													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					>

						<Route path="/" view=DashOptions/>
					</Route>
					<Route path="/test" view=TestPage/>
					<Route path="/*any" view=NotFound/>
				</Routes>
			</main>
		</Router>
	}
}

#[server]
async fn get_current_user(token: Option<String>) -> Result<Option<Usr>, ServerFnError<LoginError>>
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
			return Err(ServerFnError::WrappedServerError(LoginError::NoUserFound));
		}
		let user = user.unwrap();
		let user = user.unwrap();

		if user.id != data.claims.id || user.email != data.claims.email
		{
			return Err(ServerFnError::WrappedServerError(LoginError::NoUserFound));
		}

		let current = Usr { id: user.id, username: user.username, email: user.email };
		Ok(Some(current))
	}
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Usr
{
	id:       i32,
	username: String,
	email:    String
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
use std::str::FromStr;
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
