#[cfg(feature = "ssr")]
pub mod state;

pub(crate) mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::pages::{dash::dashboard::{DashOptions, DashboardPage},
                        home::HomePage,
                        test::TestPage,
                        user::{login::LoginPage, register::RegisterPage},
                        NotFound};
#[component]
pub fn App() -> impl IntoView
{
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
		<Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

		// sets the document title
		<Title text="Welcome to Leptos"/>

		// content for this welcome page
		<Router>
			<main>
				<Routes>
					<Route path="" view=HomePage/>
					<Route path="/login" view=LoginPage/>
					<Route path="/register" view=RegisterPage/>
					<Route path="/dash" view=DashboardPage>
						<Route path="/" view=DashOptions/>
					</Route>
					<Route path="/test" view=TestPage/>
					<Route path="/*any" view=NotFound/>
				</Routes>
			</main>
		</Router>
	}
}
