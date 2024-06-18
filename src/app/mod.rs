pub mod navbar;
pub mod sidebar;

#[cfg(feature = "ssr")]
pub mod state;

pub(crate) mod pages;

use leptos::*;

#[derive(Clone, Debug)]
pub struct SidebarSignal
{
	pub show:     ReadSignal<bool>,
	pub set_show: WriteSignal<bool>
}

impl SidebarSignal
{
	pub fn new(show: ReadSignal<bool>, set_show: WriteSignal<bool>) -> Self
	{
		Self { show,
		       set_show }
	}
}

#[component]
pub fn App() -> impl IntoView
{
	use std::rc::Rc;

	use leptos_meta::*;
	use leptos_router::*;
	use leptos_use::{use_cookie, utils::FromToStringCodec};
	use sidebar::Sidebar;

	use crate::{app::{navbar::NavBar,
	                  pages::{dash::dashboard::DashboardPage,
	                          home::HomePage,
	                          test::TestPage,
	                          user::{login::LoginPage, register::RegisterPage},
	                          NotFound},
	                  sidebar::SidebarElement},
	            server_fns::user::current::get_current_user};
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	let (sidebar_signal, set_sidebar_signal) = create_signal(false);
	let sidebar_signal = SidebarSignal::new(sidebar_signal, set_sidebar_signal);

	// Determine if there is a logged in user cookie
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// Needs to be a blocking resource because we need to wait for the cookie result before use in view
	let usr = create_blocking_resource(move || access_token.get(), get_current_user);
	let sidebar_elements: Vec<Rc<SidebarElement>> = vec![
	                                                     Rc::new(SidebarElement { label: "Employees".to_string(), content: "/emp".to_string() }),
	                                                     Rc::new(SidebarElement { label: "Departments".to_string(), content: "/deps".to_string() }),
	                                                     Rc::new(SidebarElement { label: "Profile".to_string(), content: "/user/profile".to_string() }),
	                                                     Rc::new(SidebarElement { label: "Settings".to_string(), content: "/user/settings".to_string() }),
	];

	view! {
		<Stylesheet href="https://unpkg.com/leaflet@1.9.3/dist/leaflet.css"/>
		<Script src="https://unpkg.com/leaflet@1.9.3/dist/leaflet.js"/>
		<Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

		// sets the document title
		<Title text="Welcome to AppName"/>

		<NavBar set_sidebar_signal=set_sidebar_signal/>

		<Sidebar sidebar_signal=sidebar_signal sidebar_elements=sidebar_elements/>

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
