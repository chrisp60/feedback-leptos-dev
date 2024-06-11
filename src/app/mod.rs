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

	use crate::{app::pages::{dash::dashboard::DashboardPage,
	                         home::HomePage,
	                         user::{login::LoginPage, register::RegisterPage}},
	            server_fns::user::authenticate::UserAuth};

	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");
	let auth_action = create_server_action::<UserAuth>();

	let err = Signal::derive(move || {
		(auth_action.value())().map_or("".to_owned(), |result| {
			                       match result
			                       {
				                       Ok(a) =>
			                           {
				                           return "Success:- User authenticated".to_string();
			                           }
			                           Err(err) =>
			                           {
				                           let e = format!("{:?}", err);

				                           if e.contains("NoUserFound")
				                           {
					                           return "Error:- Unable to find a user with those credentials. Please check and try again!".to_string();
				                           }
				                           else
				                           {
					                           return "Error:- Unknown error occurred.".to_string();
				                           }
			                           }
			                       }
		                       })
	});

	// // a signal that controls how many cat pics we want
	// let (how_many_cats, set_how_many_cats) = create_signal(1);

	// // create a resource that will refetch whenever `how_many_cats` changes
	// let cats = Resource::new(move || how_many_cats.get(), fetch_cat_picture_urls);
	// assert_eq!(cats.get(), Some(vec!["1".to_string()]));
	// set_how_many_cats(2);
	// assert_eq!(cats.get(), Some(vec!["2".to_string()]));

	// println!("cats = {:?}", cats.get());

	let (user_update_ready, trigger_user) = create_signal(false);

	let usr = create_blocking_resource(
	                                   || (),
	                                   |_| {
		                                   async {
			                                   get_current_user().await
			                                   //   Ok(Some(Usr { id: 1, username: "test".to_string(), email: "".to_string() }))
			                                   //   as Result<Option<Usr>, ServerFnError<LoginError>>
		                                   }
	                                   }
	);

	// create_effect(move |_| usr.and_then(|u: &Option<Usr>| println!("user resource is = {:?}", u)));

	create_effect(move |_| trigger_user.set(true));

	let username = Signal::derive(move || usr.get().map(|data| data.unwrap().map_or("".to_string(), |usr| usr.username.clone())));

	// println!("user resource = {:?}", usr());
	// let usr = move || Some(());
	view! {
		<Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

		// sets the document title
		<Title text="Welcome to Leptos"/>

		// content for this welcome page
		<Router>
			<main>
				<Routes>
					<Route path="" view=HomePage/>
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
														{"rr is none - should be showing login page"}
													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					/>

					// <Route
					// path="/auth"
					// view=move || {
					// view! {
					// // only show the outlet if data have loaded
					// <Suspense>
					// {move || {
					// usr.get()
					// .map(|data| {
					// let rr = data.unwrap();
					// println!("user resource in auth route = {:?}", rr);
					// view! {
					// <Show when=move || rr.is_some()>
					// <Redirect path="/dashboard"/>
					// </Show>
					// }
					// })
					// }}

					// </Suspense>
					// <Outlet/>
					// }
					// }
					// >

					// <Route
					// path="login"
					// view=move || {
					// view! {
					// <LoginPage
					// user_r_signal=user_update_ready
					// user_w_signal=trigger_user
					// />
					// }
					// }
					// />

					// <Route path="register" view=RegisterPage/>
					// </Route>
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
					/>

				// <Route path="/dash" view=DashboardPage>
				// <Route path="/" view=DashOptions/>
				// </Route>
				// <Route path="/test" view=TestPage/>
				// <Route path="/*any" view=NotFound/>
				</Routes>
			</main>
		</Router>
	}
}

async fn get_current_user() -> Result<Option<Usr>, ServerFnError<LoginError>>
{
	//Ok(Some(Usr { id: 1, username: "TestUserName".to_string(), email: "".to_string() }))
	Ok(None)
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
