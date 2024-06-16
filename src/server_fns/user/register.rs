use super::*;

#[server(RegisterUser, "/register")]
pub async fn register_user(first_name: String,
                           last_name: String,
                           username: String,
                           email: String,
                           date_of_birth: String,
                           password: String)
                           -> Result<i32, ServerFnError<UserRegistrationError>>
{
	println!("Register server function called for {} {} {} {} {} {}", first_name, last_name, username, email, date_of_birth, password);
	use actix_web::{cookie::{time::Duration, Cookie},
	                http::{header, header::HeaderValue}};
	use chrono::NaiveDate;
	use leptos_actix::ResponseOptions;
	use ls_entity::user;
	use ls_service::{UserMutation, UserQuery};

	use crate::app::state::AppState;

	// println!("Server Function Register");
	// println!("First Name: {}", first_name);
	// println!("Last Name: {}", last_name);
	// println!("Username: {}", username);
	// println!("Email: {}", email);
	// println!("Date of Birth: {}", date_of_birth);
	// println!("Password: {}", password);

	// check inputs are valid
	let checks = check_inputs(first_name.clone(), last_name.clone(), username.clone(), email.clone(), date_of_birth.clone(), password.clone()).await;
	if checks.is_err()
	{
		return Err(checks.err().unwrap());
	}

	let state = leptos_actix::extract().await;
	let state: AppState = state.unwrap();
	let conn = state.conn;

	let dob = NaiveDate::parse_from_str(&date_of_birth.clone(), "%Y-%m-%d").unwrap();

	let form = user::RegisterUserModel { username: username.clone(),
	                                     first_name,
	                                     last_name,
	                                     email: email.clone(),
	                                     date_of_birth: dob,
	                                     password: password.clone() };

	let create_reply = UserMutation::create_new_user(&conn, form).await;

	let mut id: i32 = -1;

	if create_reply.is_err()
	{
		let error = create_reply.err().unwrap();
		match error
		{
			ls_service::sea_orm::DbErr::Query(db_error) =>
			{
				let error = db_error.to_string();
				if error.contains("username")
				{
					return Err(ServerFnError::WrappedServerError(UserRegistrationError::UsernameAlreadyInUse));
				}
				else if error.contains("email")
				{
					return Err(ServerFnError::WrappedServerError(UserRegistrationError::EmailAlreadyInUse));
				}
			}
			_ => return Err(ServerFnError::WrappedServerError(UserRegistrationError::DatabaseError))
		}
	}
	else if let migration::sea_orm::ActiveValue::Set(value) = create_reply.unwrap().id
	{
		id = value;
	}

	let reply = UserQuery::authenticate_user(&conn, &username, &password).await;
	if reply.is_err()
	{
		// println!("Error: {}", reply);
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::DatabaseError));
	}
	else
	{
		let reply = reply.unwrap();

		// println!("Reply: {:?}", reply);
		let response = expect_context::<ResponseOptions>();

		let cookie = Cookie::build("leptos_access_token", reply.clone()).path("/").http_only(true).max_age(Duration::minutes(10)).finish();

		println!("Registration is setting up cookie");
		if let Ok(cookie) = HeaderValue::from_str(cookie.to_string().as_str())
		{
			response.insert_header(header::SET_COOKIE, cookie);
		}

		leptos_actix::redirect("/dashboard")
	};

	Ok(id)
}

#[derive(Debug, Clone)]
pub enum UserRegistrationError
{
	UsernameAlreadyInUse,
	EmailAlreadyInUse,
	InvalidDateOfBirth,
	InvalidUsername,
	InvalidNameLength,
	InvalidEmail,
	PasswordTooShort,
	PasswordNoLowercase,
	PasswordNoUppercase,
	PasswordNoNumber,
	PasswordNoSymbol,
	DatabaseError
}

impl std::fmt::Display for UserRegistrationError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			UserRegistrationError::UsernameAlreadyInUse => write!(f, "Username already in use"),
			UserRegistrationError::EmailAlreadyInUse => write!(f, "Email already in use"),
			UserRegistrationError::InvalidDateOfBirth => write!(f, "Invalid Date of Birth"),
			UserRegistrationError::InvalidUsername => write!(f, "Invalid Username"),
			UserRegistrationError::InvalidNameLength => write!(f, "Invalid Name Length"),
			UserRegistrationError::InvalidEmail => write!(f, "Invalid Email"),
			UserRegistrationError::PasswordTooShort => write!(f, "Password too short"),
			UserRegistrationError::PasswordNoLowercase =>
			{
				write!(f, "Password must contain at least 1 lowercase letter")
			}
			UserRegistrationError::PasswordNoUppercase =>
			{
				write!(f, "Password must contain at least 1 uppercase letter")
			}
			UserRegistrationError::PasswordNoNumber =>
			{
				write!(f, "Password must contain at least 1 number")
			}
			UserRegistrationError::PasswordNoSymbol =>
			{
				write!(f, "Password must contain at least 1 symbol (!@#$%^&*)")
			}
			UserRegistrationError::DatabaseError => write!(f, "Database Error")
		}
	}
}

impl FromStr for UserRegistrationError
{
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		match s
		{
			"UsernameAlreadyInUse" => Ok(UserRegistrationError::UsernameAlreadyInUse),
			"EmailAlreadyInUse" => Ok(UserRegistrationError::EmailAlreadyInUse),
			"InvalidDateOfBirth" => Ok(UserRegistrationError::InvalidDateOfBirth),
			"InvalidUsername" => Ok(UserRegistrationError::InvalidUsername),
			"InvalidNameLength" => Ok(UserRegistrationError::InvalidNameLength),
			"InvalidEmail" => Ok(UserRegistrationError::InvalidEmail),
			"PasswordTooShort" => Ok(UserRegistrationError::PasswordTooShort),
			"PasswordNoLowercase" => Ok(UserRegistrationError::PasswordNoLowercase),
			"PasswordNoUppercase" => Ok(UserRegistrationError::PasswordNoUppercase),
			"PasswordNoNumber" => Ok(UserRegistrationError::PasswordNoNumber),
			"PasswordNoSymbol" => Ok(UserRegistrationError::PasswordNoSymbol),
			"DatabaseError" => Ok(UserRegistrationError::DatabaseError),
			_ => Err(())
		}
	}
}

#[server]
async fn check_inputs(first_name: String,
                      last_name: String,
                      username: String,
                      email: String,
                      date_of_birth: String,
                      password: String)
                      -> Result<(), ServerFnError<UserRegistrationError>>
{
	// check password meets requirements
	if password.len() < 8
	{
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::PasswordTooShort));
	}
	else if !password.contains(char::is_lowercase)
	{
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::PasswordNoLowercase));
	}
	else if !password.contains(char::is_uppercase)
	{
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::PasswordNoUppercase));
	}
	else if !password.contains(char::is_numeric)
	{
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::PasswordNoNumber));
	}
	else if !password.contains(|c: char| "(!@#£$%^&*)".contains(c))
	{
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::PasswordNoSymbol));
	}

	// check age meets requirements
	let day: u32 = date_of_birth.split("-").collect::<Vec<&str>>()[2].parse::<u32>().unwrap();
	let month: u32 = date_of_birth.split("-").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
	let year: i32 = date_of_birth.split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();

	use chrono::{Datelike, Utc};

	let now = Utc::now();
	let current_year = now.year();
	let current_month = now.month();
	let current_day = now.day();

	let age = current_year - year;

	match age.cmp(&18)
	{
		std::cmp::Ordering::Less =>
		{
			return Err(ServerFnError::WrappedServerError(UserRegistrationError::InvalidDateOfBirth));
		}
		std::cmp::Ordering::Equal =>
		{
			if month > current_month || (month == current_month && day > current_day)
			{
				return Err(ServerFnError::WrappedServerError(UserRegistrationError::InvalidDateOfBirth));
			}
		}
		std::cmp::Ordering::Greater =>
		{}
	}

	// check username is valid length
	if username.len() < 6 || username.len() > 30
	{
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::InvalidUsername));
	}

	// check first name is at least 2 characters long
	if first_name.len() < 2 || last_name.len() < 2
	{
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::InvalidNameLength));
	}

	// TODO: setup sending email to verify email address

	// check email is valid
	if !email.contains("@") || !email.contains(".")
	{
		return Err(ServerFnError::WrappedServerError(UserRegistrationError::InvalidEmail));
	}

	Ok(())
}
