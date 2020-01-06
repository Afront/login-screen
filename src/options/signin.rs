use crate::Result;
pub use crate::io::{PasswordStep, password_prompt};
use crate::{hash,LoginResult};
use promptly::prompt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
	id: String,
	password: String,
}

pub async fn signin() -> Result<LoginResult> {
	loop {
		print!("\x1B[2J");
		let id: String = prompt("Please enter your username or your email");

		let user = User {
			id: id,
			password: hash(password_prompt(PasswordStep::First))	
		};
		let user_json = serde_json::to_string(&user)?;
		println!("{:?}", user_json);
//				send_json(user_json).await?;
		break; //For testing, to prevent warning since this part is not finished yet
	}
	Ok(LoginResult::AuthCode(String::from("some_auth_code")))
}