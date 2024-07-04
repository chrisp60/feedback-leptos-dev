use super::*;

#[island]
pub fn NavBar() -> impl IntoView {
    use crate::app::sidebar::Sidebar;
    let showing = RwSignal::<bool>::default();

    view! {
        <div class="flex flex-row">
            <label for="burger_button" class="ml-3 mt-0.5">
                <svg viewBox="0 0 100 60" class="dark:fill-gray-400 fill-gray-900 w-5 h-5">
                    <rect class="fill-secondary-400" width="100" height="20"></rect>
                    <rect class="fill-secondary-300" y="30" width="100" height="20"></rect>
                    <rect class="fill-secondary-400" y="60" width="100" height="20"></rect>
                </svg>
            </label>
            <button id="burger_button" on:click=move |_| showing.set(!showing.get())></button>

            <a href="/" class="font-bold text-xl text-left ml-5">
                "AppName"
            </a>
        </div>

        <Show when=showing>
            <Sidebar/>
        </Show>
    }
}

#[component]
fn BurgerPlaceholder() -> impl IntoView {
    view! {
        <div class="bg-primary-900 text-white">
            <span>
                <button class="ml-3 mt-0.5">
                    <span>
                        <svg viewBox="0 0 100 60" class="dark:fill-gray-400 fill-gray-900 w-5 h-5">
                            <rect class="fill-secondary-400" width="100" height="20"></rect>
                            <rect class="fill-secondary-300" y="30" width="100" height="20"></rect>
                            <rect class="fill-secondary-400" y="60" width="100" height="20"></rect>
                        </svg>
                    </span>
                </button>
                <a href="/" class="font-bold text-xl text-left ml-5">
                    "AppName"
                </a>
            </span>
        </div>
    }
}
