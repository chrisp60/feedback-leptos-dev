pub mod navbar;
pub mod sidebar;

#[cfg(feature = "ssr")]
pub mod state;

pub(crate) mod pages;

use leptos::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct GlobalState
{
	show_sidebar: bool
}

#[component]
pub fn App() -> impl IntoView
{
	use leptos_meta::*;
	use leptos_router::*;
	use leptos_use::{use_cookie, utils::FromToStringCodec};
	use sidebar::Sidebar;

	use crate::{app::{navbar::NavBar,
	                  pages::{dash::dashboard::DashboardPage,
	                          home::HomePage,
	                          test::TestPage,
	                          user::{login::LoginPage, register::RegisterPage},
	                          NotFound}},
	            server_fns::user::current::get_current_user};

	// provide_context(create_rw_signal(GlobalState::default()));
	let sidebar_state = create_rw_signal(false);

	// provide_context(sidebar_state);

	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	// Determine if there is a logged in user cookie
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// Needs to be a blocking resource because we need to wait for the cookie result before use in view
	let usr = create_blocking_resource(move || access_token.get(), get_current_user);

	view! {
		<Stylesheet href="https://unpkg.com/leaflet@1.9.3/dist/leaflet.css"/>
		<Script src="https://unpkg.com/leaflet@1.9.3/dist/leaflet.js"/>
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
					/>

					<Route path="/test" view=TestPage/>
					<Route path="/*any" view=NotFound/>
				</Routes>
			</main>
		</Router>
	}
}
