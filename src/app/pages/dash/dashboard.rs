use leptos::{html::Div, *};
use leptos_meta::Title;
use leptos_use::{on_click_outside, use_cookie, utils::FromToStringCodec};

#[component]
pub fn DashboardPage() -> impl IntoView
{
	view! {
		<p class="h0 m-t-10 text-center">"Dashboard"</p>

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
		<div class="container mx-auto columns-1 text-center mt-10">

			<div>
				<button class="std-btn mt-3" on:click=move |_| reset()>
					Employees
				</button>
			</div>
			<div>
				<button class="std-btn mt-3" on:click=move |_| increase()>
					Duties
				</button>
			</div>
			<div>
				<button class="std-btn mt-3" on:click=move |_| increase()>
					Profile
				</button>
			</div>
			<div>
				<button class="std-btn mt-3" on:click=move |_| increase()>
					Settings
				</button>
			</div>
		</div>

		<div class="mt-10">
			<p>
				Counter: {move || counter.get().map(|c| c.to_string()).unwrap_or("—".to_string())}
			</p>
			<button class="mt-3" on:click=move |_| reset()>
				Reset
			</button>
			<button class="mt-3" on:click=move |_| increase()>
				+
			</button>
		</div>
	}
}
