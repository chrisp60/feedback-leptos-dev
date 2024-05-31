use super::*;

#[server(RegisterUser, "/api/register")]
pub async fn register_user(first_name: String,
                           last_name: String,
                           username: String,
                           email: String,
                           date_of_birth: String,
                           password: String)
                           -> Result<(), ServerFnError<UserRegistrationError>>
{
	println!("Register server function called for {} {} {} {} {} {}", first_name, last_name, username, email, date_of_birth, password);
	leptos_actix::redirect("/dash");
	Ok(())
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
