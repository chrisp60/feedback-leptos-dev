use super::*;

#[server(UserLogin, "/api/login")]
pub async fn login(identity: String, password: String) -> Result<(), ServerFnError<LoginError>>
{
	println!("Login server function called for {} with {}", identity, password);
	leptos_actix::redirect("/dash");
	Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LoginError
{
	NoUserFound
}

impl std::fmt::Display for LoginError
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			LoginError::NoUserFound => write!(f, "NoUserFound")
		}
	}
}

impl FromStr for LoginError
{
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		match s
		{
			"NoUserFound" => Ok(LoginError::NoUserFound),
			_ => Err("Invalid LoginError".to_string())
		}
	}
}
