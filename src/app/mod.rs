#[cfg(feature = "ssr")]
pub mod jwt;
#[cfg(feature = "ssr")]
pub mod state;

pub mod dark_mode;
pub mod navbar;
pub mod pages;
pub mod sidebar;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_cookie, utils::FromToStringCodec};

use crate::{
    app::{
        dark_mode::DarkModeToggle,
        navbar::NavBar,
        pages::{
            dash::dashboard::DashboardPage,
            home::HomePage,
            user::{login::LoginPage, register::RegisterPage},
            NotFound,
        },
    },
    server_fns::user::current::get_current_user,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Determine if there is a logged in user cookie
    let (token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

    // Needs to be a blocking resource because we need to wait for the cookie result
    // before use in view
    let user = create_blocking_resource(token, get_current_user);

    let logged_in = move || {
        user.and_then(Option::is_some)
            .transpose()
            .unwrap_or_default()
            .unwrap_or_default()
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

        // sets the document title
        <Title text="Welcome to AppName"/>

        <div class="flex flex-row bg-primary-900 dark:text-primary-300">
            <div class="">
                <NavBar/>
            </div>
            <div class="darkmode-toggle-position">
                <DarkModeToggle/>
            </div>
        </div>

        <main>
            <Router fallback=move || view! { <NotFound/> }>
                <Routes>
                    <Route
                        path="/"
                        view=move || {
                            view! {
                                <Suspense>
                                    <Show when=logged_in fallback=LoginPage>
                                        <Outlet/>
                                    </Show>
                                </Suspense>
                            }
                        }
                    >

                        <Route path="dashboard" view=DashboardPage/>
                        <Route path="register" view=RegisterPage/>
                        <Route path="login" view=LoginPage/>
                        <Route path="" view=HomePage/>
                    </Route>
                </Routes>
            </Router>
        </main>
    }
}
