use log::info;
use web_sys::HtmlButtonElement;

use super::*;
use crate::server_fns::user::login::UserLogin;

#[component]
pub fn LoginPage() -> impl IntoView
{
	view! {
		<div class="bg-primary-900 text-white">
			<div class="">
				<a href="/" class="font-bold text-xl text-left ml-10 ">
					"LeptosDev"
				</a>
			</div>
		</div>

		<Title text="Login"/>

		<p class="h0 m-t-10 text-center">"Login"</p>
		// <Log/>

		// <Show when=move || err.get().contains("Success")>
		// <div class="txt-success text-center font-bold mt-10">{err}</div>
		// </Show>
		// <Show when=move || err.get().contains("Error")>
		// <div class="txt-error text-center font-bold mt-10">{err}</div>
		// </Show>

		<Counters/>
	}.into_view()
}

#[island]
fn Counters() -> impl IntoView
{
	let user_login_action = create_server_action::<UserLogin>();

	let err = Signal::derive(move || {
		(user_login_action.value())().map_or("".to_owned(), |result| {
			                             match result
			                             {
				                             Ok(a) =>
			                                 {
				                                 return "Success:- User authenticated".to_string();
			                                 }
			                                 Err(err) =>
			                                 {
				                                 let e = format!("{:?}", err);

				                                 if e.contains("NoUserFound")
				                                 {
					                                 return "Error:- Unable to find a user with those credentials. Please check and try again!".to_string();
				                                 }
				                                 else
				                                 {
					                                 return "Error:- Unknown error occurred.".to_string();
				                                 }
			                                 }
			                             }
		                             })
	});
	// Creates a reactive value to update the button
	let (count, set_count) = create_signal(0);
	let on_click = move |_| {
		set_count.update(|count| {
			         if *count == 0
			         {
				         *count += 1;
			         }
			         else
			         {
				         *count -= 1;
			         }
		         });
	};

	let show_password = Signal::derive(move || {
		if count.get() == 0
		{
			"password"
		}
		else
		{
			"text"
		}
	});

	view! {
		<Show when=move || err.get().contains("Success")>
			<div class="txt-success text-center font-bold mt-10">{err}</div>
		</Show>
		<Show when=move || err.get().contains("Error")>
			<div class="txt-error text-center font-bold mt-10">{err}</div>
		</Show>

		<div class="container mx-auto columns-1 text-center mt-10">
			<ActionForm action=user_login_action>
				<div>
					<label class="input-label" for="identity">
						"Username or Email"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type="text"
						class="ml-9"
						name="identity"
						id="identity"
						required
					/>
				</div>

				<div>
					<label class="input-label" for="password">
						"Password"
					</label>
				</div>

				<div>
					<input
						class="input-fields"
						type=show_password
						class="ml-10"
						name="password"
						id="password"
						required
					/>
				</div>

				<button class="std-btn" type="button" on:click=on_click>
					"Click Me: "
				</button>

				<div>
					<button class="std-btn" type="submit">
						"Login"
					</button>
				</div>
			</ActionForm>
		</div>
	}
}

// <div class="container mx-auto columns-1 text-center mt-10">
// 	<ActionForm action=user_login_action>

// 		<div>
// 			<label class="input-label" for="identity">
// 				"Username or Email"
// 			</label>
// 		</div>
// 		<div>
// 			<input
// 				class="input-fields"
// 				type="text"
// 				class="ml-9"
// 				name="identity"
// 				id="identity"
// 				required
// 			/>
// 		</div>

// 		<br/>

// 		<div>
// 			<label class="input-label" for="password">
// 				"Password"
// 			</label>
// 		</div>
// 		<div>
// 			<input
// 				class="input-fields"
// 				type="text"
// 				class="ml-10"
// 				name="password"
// 				id="password"
// 				required
// 			/>
// 		</div>

// 		<button type="button" class="std-btn" on:click=on_click>
// 			"Click Me: "
// 		</button>
// 		{move || { show_password.get() }}
// 		<br/>

// 		<div>
// 			<button class="std-btn" type="submit">
// 				"Login"
// 			</button>
// 		</div>
// 	</ActionForm>
// 	<br/>

// </div>

// #[island]
// fn Log() -> impl IntoView
// {
// 	let user_login_action = create_server_action::<UserLogin>();

// 	let err = Signal::derive(move || {
// 		(user_login_action.value())().map_or("".to_owned(), |result| {
// 			                             match result
// 			                             {
// 				                             Ok(a) =>
// 			                                 {
// 				                                 return "Success:- User authenticated".to_string();
// 			                                 }
// 			                                 Err(err) =>
// 			                                 {
// 				                                 let e = format!("{:?}", err);

// 				                                 if e.contains("NoUserFound")
// 				                                 {
// 					                                 return "Error:- Unable to find a user with those credentials. Please check and try again!".to_string();
// 				                                 }
// 				                                 else
// 				                                 {
// 					                                 return "Error:- Unknown error occurred.".to_string();
// 				                                 }
// 			                                 }
// 			                             }
// 		                             })
// 	});

// 	let mut show_password = false;
// 	let (read_ptype, write_ptype) = create_signal("password");

// 	view! {
// 		<Show when=move || err.get().contains("Success")>
// 			<div class="txt-success text-center font-bold mt-10">{err}</div>
// 		</Show>
// 		<Show when=move || err.get().contains("Error")>
// 			<div class="txt-error text-center font-bold mt-10">{err}</div>
// 		</Show>

// 		<div class="container mx-auto columns-1 text-center mt-10">
// 			<ActionForm action=user_login_action>

// 				<div>
// 					<label class="input-label" for="identity">
// 						"Username or Email"
// 					</label>
// 				</div>
// 				<div>
// 					<input
// 						class="input-fields"
// 						type="text"
// 						class="ml-9"
// 						name="identity"
// 						id="identity"
// 						required
// 					/>
// 				</div>

// 				<br/>

// 				<div>
// 					<label class="input-label" for="password">
// 						"Password"
// 					</label>
// 				</div>
// 				<div>
// 					<input
// 						class="input-fields"
// 						type=read_ptype
// 						class="ml-10"
// 						name="password"
// 						id="password"
// 						required
// 					/>
// 				</div>

// 				<button
// 					class="text-xs sm-btn"
// 					type="button"
// 					on:click=move |_| {
// 						show_password = !show_password;
// 						if show_password {
// 							write_ptype.set("text")
// 						} else {
// 							write_ptype.set("password")
// 						}
// 					}
// 				>

// 					"Show Password"
// 				</button>

// 				<br/>
// 				<br/>

// 				<div>
// 					<button class="std-btn" type="submit">
// 						"Login"
// 					</button>
// 				</div>
// 			</ActionForm>
// 		</div>
// 	}
// }
