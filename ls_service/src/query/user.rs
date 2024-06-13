use ls_entity::{user, user::Entity as User};
use sea_orm::*;
use sha256::digest;

use crate::utils::jwt::encode_jwt;

pub struct UserQuery;

impl UserQuery
{
	// Find a user by id
	pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr>
	{
		User::find_by_id(id).one(db).await
	}

	// Find user by email
	pub async fn find_user_by_email(db: &DbConn, email: &str) -> Result<Option<user::Model>, DbErr>
	{
		User::find().filter(user::Column::Email.eq(email)).one(db).await
	}

	// Find user by username
	pub async fn find_user_by_username(db: &DbConn, username: &str) -> Result<Option<user::Model>, DbErr>
	{
		User::find().filter(user::Column::Username.eq(username)).one(db).await
	}

	// Find all users
	pub async fn find_all_users(db: &DbConn) -> Result<Vec<user::Model>, DbErr>
	{
		User::find().all(db).await
	}

	/// Find a user by username or email and verifies password match returning an encoded JWT token.
	pub async fn authenticate_user(db: &DbConn, identity: &str, password: &str) -> Result<String, DbErr>
	{
		println!("Authenticating user");
		println!("Identity: {}", identity);
		println!("Password: {}", password);

		let password_hash = digest(password);
		println!("Password hash: {}", password_hash);

		let user = User::find().filter(Condition::all().add(user::Column::PasswordHash.eq(password_hash))
		                                               .add(Condition::any().add(user::Column::Username.eq(identity)).add(user::Column::Email.eq(identity))))
		                       .one(db)
		                       .await?;

		if user.is_none()
		{
			Err(DbErr::RecordNotFound("User not found".to_string()))
		// Ok("User not found".to_string())
		}
		else
		{
			let token = encode_jwt(user.as_ref().unwrap().email.clone(), user.as_ref().unwrap().id).expect("Could not encode JWT");
			println!("Token: {}", token);
			Ok(token)
		}
	}

	/// If ok, returns (user models, num pages).
	pub async fn find_users_in_page(db: &DbConn, page: u64, users_per_page: u64) -> Result<(Vec<user::Model>, u64), DbErr>
	{
		// Setup paginator
		let paginator = User::find().order_by_asc(user::Column::Id).paginate(db, users_per_page);
		let num_pages = paginator.num_pages().await?;

		// Fetch paginated posts
		paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
	}
}
