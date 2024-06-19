use super::*;

#[component]
pub fn NavBar() -> impl IntoView
{
	use leptos_use::{use_cookie, utils::FromToStringCodec};

	use crate::server_fns::user::current::get_current_user;

	// Determine if there is a logged in user cookie
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// Needs to be a local resource so that it only gets created once
	let usr = create_resource(move || access_token.get(), get_current_user);

	let show_signal = create_rw_signal(false);

	view! {
		<Suspense fallback=move || {
			view! {
				<div class="bg-primary-900 text-white">
					<BurgerPlaceholder/>
				</div>
			}
		}>
			<div>
				<div>
					{move || {
						usr.get()
							.map(|data| {
								let rr = data.unwrap();
								view! {
									<Show
										when=move || rr.is_some()
										fallback=move || {
											view! {
												<div class="bg-primary-900 text-white">
													<BurgerPlaceholder/>
												</div>
											}
										}
									>

										<NavBarIsland show_signal/>
									</Show>
								}
							})
					}}

				</div>

			</div>
		</Suspense>
	}
}

#[island]
fn NavBarIsland(show_signal: RwSignal<bool>) -> impl IntoView
{
	use crate::app::sidebar::Sidebar;

	let update_signal = move || show_signal.set(!show_signal.get());

	view! {
		<div class="bg-primary-900 text-white">

			<div class="flex flex-row">
				<label for="burger_button" class="ml-3 mt-0.5">
					<svg viewBox="0 0 100 60" class="dark:fill-gray-400 fill-gray-900 w-5 h-5">
						<rect class="fill-secondary-400" width="100" height="20"></rect>
						<rect class="fill-secondary-300" y="30" width="100" height="20"></rect>
						<rect class="fill-secondary-400" y="60" width="100" height="20"></rect>
					</svg>
				</label>
				<button id="burger_button" on:click=move |_| update_signal()></button>

				<a href="/" class="font-bold text-xl text-left ml-5">
					"AppName"
				</a>
			</div>

		</div>
		<div>
			<Show when=move || { show_signal.get() } fallback=|| view! { <div></div> }>
				<Sidebar/>
			</Show>
		</div>
	}
}

#[component]
fn BurgerPlaceholder() -> impl IntoView
{
	view! {
		<div>
			<span>
				<button class="ml-3 mt-0.5">
					<BurgerGraphic/>
				</button>
				<a href="/" class="font-bold text-xl text-left ml-5">
					"AppName"
				</a>
			</span>
		</div>
	}.into_view()
}

#[component]
fn BurgerGraphic() -> impl IntoView
{
	view! {
		<span>
			<svg viewBox="0 0 100 60" class="dark:fill-gray-400 fill-gray-900 w-5 h-5">
				<rect class="fill-secondary-400" width="100" height="20"></rect>
				<rect class="fill-secondary-300" y="30" width="100" height="20"></rect>
				<rect class="fill-secondary-400" y="60" width="100" height="20"></rect>
			</svg>
		</span>
	}.into_view()
}
