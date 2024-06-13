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

		<Log/>
	}.into_view()
}

#[island]
fn Log() -> impl IntoView
{
	let user_login_action = create_server_action::<UserLogin>();

	let err = Signal::derive(move || {
		(user_login_action.value())().map_or("".to_owned(), |result| {
			                             match result
			                             {
				                             Ok(_) =>
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
	let (show, set_show) = create_signal(0);
	let on_click = move |_| {
		set_show.update(|show| {
			        if *show == 0
			        {
				        *show += 1;
			        }
			        else
			        {
				        *show -= 1;
			        }
		        });
	};

	// assign the value of the password field type
	let show_password = Signal::derive(move || {
		if show.get() == 0
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

				<div class="mt-3">
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

				<div>
					<button class="sm-btn" type="button" on:click=on_click>
						"Show Password"
					</button>
				</div>

				<div class="mt-5">
					<button class="std-btn" type="submit">
						"Login"
					</button>
				</div>
			</ActionForm>
		</div>
	}
}
