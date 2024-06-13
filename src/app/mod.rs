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

	use crate::{app::pages::{dash::dashboard::{DashOptions, DashboardPage},
	                         home::HomePage,
	                         test::TestPage,
	                         user::{login::LoginPage, register::RegisterPage},
	                         NotFound},
	            server_fns::user::current::get_current_user};

	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	let usr = create_blocking_resource(move || access_token.get(), get_current_user);

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
