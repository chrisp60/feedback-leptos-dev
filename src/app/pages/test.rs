use leptos::*;

/// Renders the home page of your application.
#[component]
pub(crate) fn TestPage() -> impl IntoView
{
	let files = ["a.txt", "b.txt", "c.txt"];
	let labels = files.iter().copied().map(Into::into).collect();
	let tabs = move || {
		files.into_iter()
		     .enumerate()
		     .map(|(index, filename)| {
			     let content = std::fs::read_to_string(filename).unwrap();
			     view! {
					 <Tab index>
						 <div>
							 <h2>{filename.to_string()}</h2>
							 <p>{content}</p>
						 </div>
					 </Tab>
				 }
		     })
		     .collect_view()
	};

	view! {
		<h1 class="text-center">"Welcome to Leptos!"</h1>

		<Counter/>

		<h2 class="text-center">"Click any of the tabs below to read a recipe."</h2>
		<Tabs labels>
			<div>{tabs()}</div>
		</Tabs>
	}
}

#[island]
fn Counter() -> impl IntoView
{
	// Creates a reactive value to update the button
	let (count, set_count) = create_signal(0);
	let on_click = move |_| set_count.update(|count| *count += 1);

	view! {
		<div class="text-center">
			<h2>"Counter"</h2>

			<button class="std-btn" on:click=on_click>
				"Click Me: "
				{count}
			</button>
		</div>
	}
}

#[island]
fn Tab(index: usize, children: Children) -> impl IntoView
{
	let selected = expect_context::<ReadSignal<usize>>();
	view! {
		<div
			class="text-center"
			style:display=move || { if selected() == index { "block" } else { "none" } }
		>

			{children()}
		</div>
	}
}

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView
{
	let (selected, set_selected) = create_signal(0);
	provide_context(selected);

	let buttons = labels.into_iter()
	                    .enumerate()
	                    .map(|(index, label)| {
		                    view! {
								<button class="std-btn" on:click=move |_| set_selected(index)>
									{label}
								</button>
							}
	                    })
	                    .collect_view();
	view! {
		<div class="text-center space-x-4">{buttons}</div>
		{children()}
	}
}
