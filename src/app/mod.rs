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

	// Determine if there is a logged in user cookie
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// Needs to be a blocking resource because we need to wait for the cookie result before use in view
	let usr = create_blocking_resource(move || access_token.get(), get_current_user);

	view! {
		<Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

		// sets the document title
		<Title text="Welcome to AppName"/>

		<NavBar/>

		// content for this welcome page
		<Router>
			<main>
				<Routes>
					<Route
						path="/"
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
														<HomePage/>
													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					/>

					<Route
						path="/register"
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

#[island]
fn NavBar() -> impl IntoView
{
	use leptos_use::{use_cookie, utils::FromToStringCodec};

	use crate::server_fns::user::current::get_current_user;

	// Determine if there is a logged in user cookie
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// Needs to be a local resource so that it only gets created once
	let usr = create_local_resource(move || access_token.get(), get_current_user);

	view! {
		<Suspense fallback=move || {
			view! {
				<div class="bg-primary-900 text-white">
					<BurgerPlaceholder/>
				</div>
			}
		}>
			<div class="bg-primary-900 text-white">
				<div>
					{move || {
						usr.get()
							.map(|data| {
								let rr = data.unwrap();
								view! {
									<Show
										when=move || rr.is_some()
										fallback=move || {
											view! { <BurgerPlaceholder/> }
										}
									>

										<div>
											<LoggedInNavBar/>
										</div>
									</Show>
								}
							})
					}}

				</div>

			</div>
		</Suspense>
	}
}

#[island]
fn LoggedInNavBar() -> impl IntoView
{
	let (show_modal, set_show_modal) = create_signal(false);

	view! {
		<div>
			<span>
				<button
					class="ml-3 w-20"
					on:click=move |_| {
						logging::log!("show_modal is {}", show_modal());
						if show_modal() {
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
				// button back to home
				<a href="/" class="font-bold text-xl text-left ml-3 ">
					"AppName"
				</a>
			// tabs for the dashboard (Employees, Departments, etc.)

			</span>
		</div>

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

#[component]
fn BurgerPlaceholder() -> impl IntoView
{
	view! {
		<div>
			<span>
				<button class="ml-3 w-20">

					<span>
						<svg viewBox="0 0 100 60" class="dark:fill-gray-400 fill-gray-900 w-5 h-5">
							<rect class="fill-secondary-400" width="100" height="20"></rect>
							<rect class="fill-secondary-300" y="30" width="100" height="20"></rect>
							<rect class="fill-secondary-400" y="60" width="100" height="20"></rect>
						</svg>
					</span>
				</button>
				<a href="/" class="font-bold text-xl text-left ml-3 ">
					"AppName"
				</a>
			</span>
		</div>
	}.into_view()
}
