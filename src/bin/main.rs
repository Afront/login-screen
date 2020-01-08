use login_screen::Result;
use login_screen::LoginResult;
use login_screen::options::{abort,help,signup,signin};
use promptly::{prompt}; //use promptly::{prompt, prompt_default};
use std::{env, process};

async fn login_screen() -> Result<LoginResult>{
	loop {
		print!("\x1B[2J");
	
		//note: for some reason, the errors do not pop up
		let login_url = env::var("LOGIN_URL").expect("LOGIN_URL must be set"); //Or SESSION_URL
		let signup_url = env::var("SIGNUP_URL").expect("SIGNUP_URL must be set"); //Or USERS_URL 
		//add URL validator...
		
		let input: String = prompt("Hello! Would you like to sign (i)n or sign (u)p? ");
		match match input.trim().to_uppercase().as_str() {
				"ABORT" | "EXIT" | "Q" | "QUIT" => abort(),
				"HELP" | "H" => help(),
				"SIGN UP" | "SIGNUP" | "REGISTER" | "U" => signup(signup_url).await,
				"SIGN IN" | "SIGNIN" | "LOGIN" | "LOG IN" | "I" => signin(login_url).await,
				_  => continue,
		} {
			Ok(LoginResult::Quit) => process::exit(0),
			Ok(LoginResult::Helped) => continue,
			Ok(LoginResult::SignedUp) => continue,
			Ok(LoginResult::AuthCode(auth_code)) => return Ok(LoginResult::AuthCode(auth_code)),
			Err(err) => println!("{:?}", err),
		};
	}
}


fn do_something(){
	unimplemented!();
}

#[tokio::main]
async fn main() -> Result<()> {
	match login_screen().await {
		Ok(_) => (),
		Err(_) => (),
	}

	do_something();
	Ok(())
}