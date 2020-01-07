use std::io::Write;
pub use error::Error;

pub mod error {
	#[derive(Debug)]
	pub enum Error {
		InvalidEmail,
		InvalidPassword,
		InvalidUsername,
	}
}

#[derive(Debug, PartialEq)]
pub enum PasswordStep {
	First,
	Second
}

pub fn password_prompt(choice: PasswordStep) -> String {
	print!("Please enter your password{}: " , if choice == PasswordStep::Second {" again"} else {""});	
	std::io::stdout().flush().unwrap();
	rpassword::read_password().unwrap()
}