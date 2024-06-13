use leptos::*;
use leptos_meta::Title;
use leptos_use::{use_cookie, utils::FromToStringCodec};

#[component]
pub fn DashboardPage() -> impl IntoView
{
	view! {
		<div class="bg-primary-900 text-white">
			<div class="">
				<a href="/" class="font-bold text-xl text-left ml-10 ">
					"LeptosDev"
				</a>
			</div>
		</div>

		<Title text="Dashboard"/>

		<h1>"Dashboard"</h1>

		<DashOptions/>
	}
}

#[island]
pub fn DashOptions() -> impl IntoView
{
	let (counter, set_counter) = use_cookie::<u32, FromToStringCodec>("counter");

	let reset = move || set_counter.set(Some(0));

	if counter.get_untracked().is_none()
	{
		reset();
	}

	let increase = move || {
		set_counter.set(counter.get().map(|c| c + 1));
	};

	view! {
		<p>Counter: {move || counter.get().map(|c| c.to_string()).unwrap_or("—".to_string())}</p>
		<button on:click=move |_| reset()>Reset</button>
		<button on:click=move |_| increase()>+</button>
	}
}
