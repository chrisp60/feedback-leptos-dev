use super::*;

#[island]
pub fn NavBar(set_sidebar_signal: WriteSignal<bool>) -> impl IntoView
{
	use leptos_use::{use_cookie, utils::FromToStringCodec};

	use crate::server_fns::user::current::get_current_user;

	// Determine if there is a logged in user cookie
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// Needs to be a local resource so that it only gets created once
	let usr = create_local_resource(move || access_token.get(), get_current_user);

	view! {
		<Suspense fallback=move || {
			view! {
				<div class="bg-primary-900 text-white">
					<BurgerPlaceholder/>
				</div>
			}
		}>
			<div class="bg-primary-900 text-white">
				<div>
					{move || {
						usr.get()
							.map(|data| {
								let rr = data.unwrap();
								view! {
									<Show
										when=move || rr.is_some()
										fallback=move || {
											view! { <BurgerPlaceholder/> }
										}
									>

										<div>
											<LoggedInNavBar set_sidebar_signal=set_sidebar_signal/>
										</div>
									</Show>
								}
							})
					}}

				</div>

			</div>
		</Suspense>
	}
}

#[component]
fn LoggedInNavBar(set_sidebar_signal: WriteSignal<bool>) -> impl IntoView
{
	let (show_modal, set_show_modal) = create_signal(false);

	view! {
		<div>
			<span>
				<button class="ml-3 w-20" on:click=move |_| {}>

					<span>
						<svg viewBox="0 0 100 60" class="dark:fill-gray-400 fill-gray-900 w-5 h-5">
							<rect class="fill-secondary-400" width="100" height="20"></rect>
							<rect class="fill-secondary-300" y="30" width="100" height="20"></rect>
							<rect class="fill-secondary-400" y="60" width="100" height="20"></rect>
						</svg>
					</span>
				</button>
				// button back to home
				<a href="/" class="font-bold text-xl text-left ml-3 ">
					"AppName"
				</a>
			// tabs for the dashboard (Employees, Departments, etc.)

			</span>
		</div>
	}
}

#[component]
fn BurgerPlaceholder() -> impl IntoView
{
	view! {
		<div>
			<span>
				<button class="ml-3 w-20">

					<span>
						<svg viewBox="0 0 100 60" class="dark:fill-gray-400 fill-gray-900 w-5 h-5">
							<rect class="fill-secondary-400" width="100" height="20"></rect>
							<rect class="fill-secondary-300" y="30" width="100" height="20"></rect>
							<rect class="fill-secondary-400" y="60" width="100" height="20"></rect>
						</svg>
					</span>
				</button>
				<a href="/" class="font-bold text-xl text-left ml-3 ">
					"AppName"
				</a>
			</span>
		</div>
	}.into_view()
}
