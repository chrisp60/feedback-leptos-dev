mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::pages::{home::HomePage, test::TestPage, NotFound};

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
					<Route path="/test" view=TestPage/>
					<Route path="/*any" view=NotFound/>
				</Routes>
			</main>
		</Router>
	}
}
