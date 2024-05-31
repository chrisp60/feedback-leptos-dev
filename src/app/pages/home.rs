use leptos::*;
use leptos_meta::Title;

#[component]
pub(crate) fn HomePage() -> impl IntoView
{
	view! {
		<div class="bg-primary-900 text-white">
			<div class="">
				<a href="/" class="font-bold text-xl text-left ml-10 ">
					"LeptosDev"
				</a>

			</div>
		</div>

		<Title text="LeptosDev"/>

		<div class="text-center margin-top-8">
			<p class="h0">"Leptos Dev!"</p>
			<h2 class="txt-primary margin-top-5">"Some text about the app"</h2>

			<h4 class="txt-tertiary margin-top-3">"More text about the app"</h4>

			<div class="margin-top-5">
				<a href="/login" class="std-btn">
					"Login"
				</a>
			</div>

			<h6 class="txt-tertiary margin-top-3">"or"</h6>

			<div class="margin-top-3">
				<a href="/register" class="std-btn" type="submit">
					"Register"
				</a>
			</div>

			<div class="margin-top-5">
				<a href="/test" class="std-btn">
					"Test Page"
				</a>
			</div>
		</div>
	}
}
