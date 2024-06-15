#[cfg(feature = "ssr")]
pub mod state;

pub(crate) mod pages;
use leptos::{html::Div, *};

#[component]
pub fn App() -> impl IntoView
{
	use leptos::provide_context;
	use leptos_meta::*;
	use leptos_router::*;
	use leptos_use::{storage::use_local_storage, use_cookie, utils::FromToStringCodec};

	use crate::{app::pages::{dash::dashboard::{DashOptions, DashboardPage},
	                         home::HomePage,
	                         test::TestPage,
	                         user::{login::LoginPage, register::RegisterPage},
	                         NotFound},
	            server_fns::user::current::get_current_user};

	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	let usr = create_resource(move || access_token.get(), get_current_user);

	create_effect(move |_| {
		let (user, set_user, _) = use_local_storage::<Usr, FromToStringCodec>("leptos_user");
		request_animation_frame(move || set_user.set(usr().unwrap().unwrap().unwrap()));

		provide_context::<Usr>(user());
	});

	view! {
		<Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

		// sets the document title
		<Title text="Welcome to Leptos"/>

		<NavBar/>

		// content for this welcome page
		<Router>
			<main>
				<Routes>
					<Route path="" view=HomePage/>
					<Route
						path="register"
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
														<RegisterPage/>
													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					/>

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
// let username = Signal::derive(move || usr.get().map(|data| data.unwrap().map_or("".to_string(), |usr| usr.username.clone())));
// async fn use_auth_user() -> Signal<Option<AuthenticatedUser>>
// {
// 	use leptos_use::{storage::use_local_storage, use_cookie, utils::FromToStringCodec};

// 	use crate::server_fns::user::current::get_current_user;

// 	(move || {
// 		if get_current_user(access_token()).unwrap().is_some()
// 		{
// 			Some(AuthenticatedUser::from_whatever(user()))
// 		}
// 		else
// 		{
// 			None
// 		}
// 	}).into_signal()
// }
use crate::server_fns::user::current::Usr;
#[derive(Debug, Clone)]
pub struct AuthenticatedUser
{
	pub user: Usr
}

impl AuthenticatedUser
{
	pub fn from_whatever(user: Usr) -> Self
	{
		Self { user }
	}
}

#[island]
fn NavBar() -> impl IntoView
{
	use leptos_use::{use_cookie, utils::FromToStringCodec};

	use crate::server_fns::user::current::get_current_user;

	// let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// let usr = create_resource(move || access_token.get(), get_current_user);

	let current_user: Option<Usr> = None;
	println!("before - current_user is {:?}", current_user);

	create_effect(move |_| {
		let user: Option<Usr> = use_context();
		request_animation_frame(move || current_user = user);
	});

	println!("after - current_user is {:?}", current_user);

	view! {
		<Suspense fallback=move || { "Loading...." }>
			<div class="bg-primary-900 text-white justify-between">
				<div>
					{move || {
						usr.get()
							.map(|data| {
								let rr = data.unwrap();
								view! {
									<Show
										when=move || rr.is_some()
										fallback=move || {
											view! { "Loading...." }
										}
									>

										<MenuModal/>
									</Show>
								}
							})
					}}

				</div>

			</div>
		</Suspense>
	}
}

#[component]
fn MenuModal() -> impl IntoView
{
	let (show_modal, set_show_modal) = create_signal(false);

	view! {
		<span class="">
			<button
				class="ml-3 w-20"
				on:click=move |_| {
					logging::log!("show_modal is {}", show_modal());
					if show_modal() == true {
						set_show_modal.set(false)
					} else {
						set_show_modal.set(true)
					}
				}
			>

				<span>
					<svg viewBox="0 0 100 60" class="dark:fill-gray-400 fill-gray-900 w-5 h-5">
						<rect class="fill-secondary-400" width="100" height="20"></rect>
						<rect class="fill-secondary-300" y="30" width="100" height="20"></rect>
						<rect class="fill-secondary-400" y="60" width="100" height="20"></rect>
					</svg>
				</span>
			</button>

		</span>

		<Suspense fallback=move || {
			"Loading...."
		}>
			{move || {
				let show = show_modal.get();
				if show {
					view! {
						<div class="std-bdr-2 mt-5 ml-5 w-60">
							<div>
								<button
									class="std-btn"
									title="Close"
									on:click=move |_| set_show_modal.set(false)
								>
									"𝖷"
								</button>
								<p class="heading">"Demo Modal"</p>
								<p>"Click outside this modal to close it."</p>
							</div>
						</div>
					}
				} else {
					view! { <div></div> }
				}
			}}

		</Suspense>
	}
}
