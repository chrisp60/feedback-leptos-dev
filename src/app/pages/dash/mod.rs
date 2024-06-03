pub mod dashboard;

use leptos::*;
use leptos_meta::Title;

use crate::server_fns::user::check_auth_token;

#[component]
pub fn AuthPage() -> impl IntoView
{
	view! {
		<div class="bg-primary-900 text-white">
			<div class="">
				<a href="/" class="font-bold text-xl text-left ml-10 ">
					"LeptosDev"
				</a>
			</div>
		</div>

		<Title text="Auth"/>

		<AuthCheck/>
	}
}

#[island]
fn AuthCheck() -> impl IntoView
{
	use leptos_router::{Outlet, Redirect};
	use leptos_use::{use_cookie, utils::FromToStringCodec};

	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	println!("AuthCheck");

	let resource = create_blocking_resource(access_token, |value| {
		async move {
			let reply = check_auth_token(value).await;

			println!("Inside resource {:?}", reply);

			match reply
			{
				Err(e) =>
				{
					println!("Inside resource error {:?}", e);
					None
				}
				Ok(s) =>
				{
					println!("Inside resource success {:?}", s);
					Some("Success:- User authenticated".to_string())
				}
			}
		}
	});

	view! {
		<Suspense fallback=|| {
			view! { <div class="container mx-auto columns-1 text-center mt-10">"Loading..."</div> }
		}>
			<Show when=move || resource.get().is_some()>
				<Redirect path="/login"/>
			</Show>

		</Suspense>
		<Outlet/>
	}
}
