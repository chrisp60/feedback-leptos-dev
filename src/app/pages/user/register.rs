use super::*;
use crate::server_fns::user::register::RegisterUser;

#[component]
pub fn RegisterPage() -> impl IntoView
{
	view! {
		<div class="bg-primary-900 text-white">
			<div class="">
				<a href="/" class="font-bold text-xl text-left ml-10 ">
					"LeptosDev"
				</a>
			</div>
		</div>

		<Title text="Register"/>

		<p class="h0 m-t-10 text-center">"Register New User"</p>

		<Reg/>
	}
}

#[island]
fn Reg() -> impl IntoView
{
	let register_user_action = create_server_action::<RegisterUser>();

	let err = Signal::derive(move || {
		(register_user_action.value())().map_or("".to_owned(), |result| {
			                                match result
			                                {
				                                Ok(_) => "Success:- New user registered".to_string(),
			                                    Err(err) =>
			                                    {
				                                    let e = format!("{:?}", err);

				                                    if e.contains("Username") && e.contains("already in use")
				                                    {
					                                    return "Error:- Username is already in use".to_string();
				                                    }
				                                    else if e.contains("Email") && e.contains("already in use")
				                                    {
					                                    return "Error:- Email address already in use".to_string();
				                                    }
				                                    else if e.contains("Date of Birth")
				                                    {
					                                    return "Error:- Invalid Date of Birth, you must be 18 years or older to use this app!".to_string();
				                                    }
				                                    else if e.contains("Username") && e.contains("Invalid")
				                                    {
					                                    return "Error:- Usernames must be between 6 and 30 characters long".to_string();
				                                    }
				                                    else if e.contains("Name") && e.contains("Length")
				                                    {
					                                    return "Error:- First and last names must be at least 2 characters long".to_string();
				                                    }
				                                    else if e.contains("Email") && e.contains("Invalid")
				                                    {
					                                    return "Error:- Invalid email address".to_string();
				                                    }
				                                    else if e.contains("Password") && e.contains("short")
				                                    {
					                                    return "Error:- Password must be at least 8 characters long".to_string();
				                                    }
				                                    else if e.contains("lowercase")
				                                    {
					                                    return "Error:- Password must contain at least 1 lowercase letter".to_string();
				                                    }
				                                    else if e.contains("uppercase")
				                                    {
					                                    return "Error:- Password must contain at least 1 uppercase letter".to_string();
				                                    }
				                                    else if e.contains("number")
				                                    {
					                                    return "Error:- Password must contain at least 1 number".to_string();
				                                    }
				                                    else if e.contains("symbol")
				                                    {
					                                    return "Error:- Password must contain at least 1 of these symbols (!@#$%^&*)".to_string();
				                                    }
				                                    else if e.contains("duplicate") && e.contains("username")
				                                    {
					                                    return "Error:- Username is already in use".to_string();
				                                    }
				                                    else if e.contains("duplicate") && e.contains("email")
				                                    {
					                                    return "Error:- Email address already in use".to_string();
				                                    }
				                                    else
				                                    {
					                                    return "Error:- Database Error".to_string();
				                                    }
			                                    }
			                                }
		                                })
	});

	let mut show_password = false;
	let (read_ptype, write_ptype) = create_signal("password");

	view! {
		<Show when=move || err.get().contains("Success")>
			<div class="txt-success text-center font-bold mt-10">{err}</div>
		</Show>
		<Show when=move || err.get().contains("Error")>
			<div class="txt-error text-center font-bold mt-10">{err}</div>
		</Show>

		<div class="container mx-auto columns-1 text-center mt-10">
			<ActionForm action=register_user_action>

				<div>
					<label class="input-label" for="username">
						"Username"
					</label>
				</div>
				<div>
					<input class="input-fields" type="text" name="username" id="username" required/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="first_name">
						"First Name"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type="text"
						name="first_name"
						id="first_name"
						required
					/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="last_name">
						"Last Name"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type="text"
						name="last_name"
						id="last_name"
						required
					/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="date_of_birth">
						"Date of Birth"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type="date"
						name="date_of_birth"
						id="date_of_birth"
						required
					/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="email">
						"Email"
					</label>
				</div>
				<div>
					<input class="input-fields" type="email" name="email" id="email" required/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="password">
						"Password"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type=read_ptype
						name="password"
						id="password"
						required
					/>

				</div>

				<div>
					<button
						class="text-xs sm-btn"
						type="button"
						on:click=move |_| {
							show_password = !show_password;
							if show_password {
								write_ptype.set("text")
							} else {
								write_ptype.set("password")
							}
						}
					>

						"Show Password"
					</button>
				</div>

				<div class="mt-5">
					<button class="std-btn" type="submit">
						"Register"
					</button>
				</div>
			</ActionForm>
		</div>
	}
}
