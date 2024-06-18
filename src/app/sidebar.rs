use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::*;

#[component]
pub fn Sidebar(sidebar_elements: Vec<Rc<SidebarElement>>) -> impl IntoView
{
	let initial_item = sidebar_elements.first()?;
	provide_context(RwSignal::new(initial_item.clone()));

	let elem_iter = SidebarIter::new(sidebar_elements);
	let elem_iter_clone = elem_iter.clone();

	Some(view! {
		<div class="side-bar">
			<For
				each=move || elem_iter_clone.clone()
				key=|element| element.label.clone()
				let:element
			>
				<div class="w-16">
					<button class="std-btn mt-5">{element.label.clone()}</button>
				</div>
			</For>
		</div>
	})
}

#[derive(Clone)]
struct SidebarIter
{
	elements: Vec<Rc<SidebarElement>>,
	index:    usize
}

impl SidebarIter
{
	fn new(elements: Vec<Rc<SidebarElement>>) -> Self
	{
		Self { elements,
		       index: 0 }
	}
}

impl Iterator for SidebarIter
{
	type Item = Rc<SidebarElement>;

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.index < self.elements.len()
		{
			let element = self.elements.get(self.index)?;
			self.index += 1;
			Some(element.clone())
		}
		else
		{
			None
		}
	}
}

#[component]
fn SidebarContent(element: Rc<SidebarElement>) -> impl IntoView
{
	let element_clone = element.clone();
	//let selected_element = expect_context::<RwSignal<Rc<SidebarElement>>>();

	view! {}
}

#[component]
pub fn MaybeHidden(#[prop(into)] hidden: Signal<bool>, children: leptos::ChildrenFn) -> impl IntoView
{
	view! {
		move || if hidden.get()
		{None::<leptos::Fragment>}
		else
		{Some(children())}
	}
}

// #[component]
// fn SidebarLabel(element: Rc<SidebarElement>) -> impl IntoView
// {
// 	const ITEM_CLASSES: &'static str = "...";
// 	const SELECTED_ITEM_CLASSES: &'static str = "...";

// 	let click_element = element.clone();
// 	let class_element = element.clone();
// 	let selected_element = expect_context::<RwSignal<Rc<SidebarElement>>>();

// 	view! {
// 		<div
// 			class=move || {
// 				if selected_element.get().label == class_element.label {
// 					SELECTED_ITEM_CLASSES
// 				} else {
// 					ITEM_CLASSES
// 				}
// 			}

// 			on:click=move |_| {
// 				if selected_element.get().label != click_element.label {
// 					selected_element.set(click_element.clone())
// 				}
// 			}
// 		>

// 			<p class="mr-2 align-right justify-self-end">{element.label.clone()}</p>
// 		</div>
// 	}
// }

#[component]
#[derive(Clone, Serialize, Deserialize)]
pub struct SidebarElement
{
	pub label:   String,
	pub content: String
}

impl SidebarElement
{
	pub fn new(label: &str, content: &str) -> Self
	{
		Self { label: label.to_string(), content: content.to_string() }
	}
}
