use leptos::*;
use leptos_meta::Title;

#[component]
pub(crate) fn HomePage() -> impl IntoView
{
	view! {
		<Title text="LeptosDev"/>

		<div class="text-center">
			<p class="h0 m-t-10">"Leptos Dev!"</p>
			<h2 class="txt-primary m-t-8">"Some text about the app"</h2>

			<h4 class="txt-tertiary m-t-6">"More text about the app"</h4>

			<div class="m-t-6 space-x-6">
				<a href="/login" class="std-btn">
					"Login"
				</a>

				<a href="/register" class="std-btn" type="submit">
					"Register"
				</a>
			</div>

			<h6 class="txt-tertiary m-t-6">"or"</h6>

			<div class="m-t-6"></div>

			<div class="m-t-6">
				<a href="/dashboard" class="std-btn">
					"Dash Page"
				</a>
			</div>
		</div>
	}
}
