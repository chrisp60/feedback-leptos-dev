use leptos::*;
use leptos_meta::Title;
use log::info;

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

		<p class="h0 mt-10 text-center">"Dashboard"</p>

		<AuthCheck/>
	}
}

#[island]
fn AuthCheck() -> impl IntoView
{
	use leptos_use::{use_cookie, utils::FromToStringCodec};

	let (counter, set_counter) = use_cookie::<u32, FromToStringCodec>("counter");

	let reset = move || set_counter.set(Some(10));

	create_effect(move |_| {
		if counter.get().is_none()
		{
			reset();
		}
	});

	let increase = move || {
		set_counter.set(counter.get().map(|c| c + 1));
	};

	view! {
		<p>Counter: {move || counter.get().map(|c| c.to_string()).unwrap_or("—".to_string())}</p>
		<button on:click=move |_| reset()>Reset</button>
		<br/>
		<button on:click=move |_| increase()>+</button>
	}
}

// #[island]
// fn AuthCheck() -> impl IntoView
// {
// 	let user = create_blocking_resource(
// 	                                    || (),
// 	                                    move |_| {
// 		                                    async {
// 			                                    let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

// 			                                    let token = access_token.get();
// 			                                    info!("token received {token:?}");
// 			                                    match token
// 			                                    {
// 				                                    Some(token) => AuthType::Authorized(token),
// 			                                        None => AuthType::UnAuthorized
// 			                                    }
// 		                                    }
// 	                                    }
// 	);

// 	println!("DashboardPage {:?}", user.get());
// }

// match user.get().unwrap_or(AuthType::UnAuthorized) {
//     AuthType::Authorized(_) => view! { <Outlet/> }.into_view(),
//     AuthType::UnAuthorized => {
//         info!("Redirect to home {:?} isloading {:?}", user.get(), user.loading().get());
//         view! { <Redirect path="/"/> }.into_view()
//     }
