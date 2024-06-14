use leptos::*;

/// Renders the home page of your application.
#[component]
pub(crate) fn TestPage() -> impl IntoView
{
	view! {
		<h1 class="text-center">"Welcome to Leptos!"</h1>

		<ShowModal/>
		<br/>
	}
}

#[island]
fn ShowModal() -> impl IntoView
{
	let (show_modal, set_show_modal) = create_signal(false);
	view! {
		<button on:click=move |_| set_show_modal.update(|value| *value = true)>"Click me"</button>

		<div>{move || show_modal()}</div>
	}
}

// #[island]
// fn Counter() -> impl IntoView
// {
// 	// Creates a reactive value to update the button
// 	let (count, set_count) = create_signal(0);
// 	let on_click = move |_| set_count.update(|count| *count += 1);

// 	view! {
// 		<div class="text-center">
// 			<h2>"Counter"</h2>

// 			<button class="std-btn" on:click=on_click>
// 				"Click Me: "
// 				{count}
// 			</button>
// 		</div>
// 	}
// }

// #[island]
// fn Counters() -> impl IntoView
// {
// 	// Creates a reactive value to update the button
// 	let (count, set_count) = create_signal(0);
// 	let on_click = move |_| {
// 		set_count.update(|count| {
// 			         if *count == 0
// 			         {
// 				         *count += 1;
// 			         }
// 			         else
// 			         {
// 				         *count -= 1;
// 			         }
// 		         });
// 	};

// 	let show_password = Signal::derive(move || {
// 		if count.get() == 0
// 		{
// 			"password"
// 		}
// 		else
// 		{
// 			"text"
// 		}
// 	});

// 	view! {
// 		<div class="text-center">
// 			<h2>"Toggle"</h2>

// 			<div>
// 				<input
// 					class="input-fields"
// 					type=show_password
// 					class="ml-10"
// 					name="password"
// 					id="password"
// 					required
// 				/>
// 			</div>

// 			<button class="std-btn" on:click=on_click>
// 				"Click Me: "
// 			</button>
// 		</div>
// 	}
// }
// #[island]
// fn Tab(index: usize, children: Children) -> impl IntoView
// {
// 	let selected = expect_context::<ReadSignal<usize>>();
// 	view! {
// 		<div
// 			class="text-center"
// 			style:display=move || { if selected() == index { "block" } else { "none" } }
// 		>

// 			{children()}
// 		</div>
// 	}
// }

// #[island]
// fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView
// {
// 	let (selected, set_selected) = create_signal(0);
// 	provide_context(selected);

// 	let buttons = labels.into_iter()
// 	                    .enumerate()
// 	                    .map(|(index, label)| {
// 		                    view! {
// 								<button class="std-btn" on:click=move |_| set_selected(index)>
// 									{label}
// 								</button>
// 							}
// 	                    })
// 	                    .collect_view();
// 	view! {
// 		<div class="text-center space-x-4">{buttons}</div>
// 		{children()}
// 	}
// }
