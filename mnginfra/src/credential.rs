//credential stores credential information for a given software in a given asset

//currently we will not implement the password part, 
//as security configurations will have to be taken into account

use std::io;



pub struct Credential{
	title: String,
	user: String,
	password: String,
	active: bool,
}


impl Credential {
    fn new(title: String, user: Option<String>, active: Option<bool>) -> Result<Self, String> {
        if title.len() == 0 {return Err("invalid credential title when creating CREDENTIAL structure".to_string())}
        Ok( Self {
			title,
			user: user.unwrap_or("".to_string()),
            password: "<not supported>".to_string(),
			active: active.unwrap_or(true),
		})
	}

    pub fn new_from_cli() -> Result<Self, String> {
        let mut title = String::new();
        let mut user  = String::new();

        loop {
            title.clear();
            user.clear();
            println!("Please input a title for the credential");
            println!("(q to quit, r to restart)");
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");
            if title.trim() == "q" {return Err("user input canceled".to_string());}
            if title.trim() == "r" {continue;}


            println!("Please input a username for the credential");
            println!("(q to quit, r to restart)");
            io::stdin()
                .read_line(&mut user)
                .expect("Failed to read line");
            if user.trim() == "q" {return Err("user input canceled".to_string());}
            if user.trim() == "r" {continue;}

            break;
        }

            match Self::new(title.clone().trim().to_string(), Some(user.clone().trim().to_string()), Some(true)) {
                Ok(d) => return Ok(d),
                Err(e) => Err(format!("[E:] {}", e),)
            }
    }

    pub fn info(&self) -> String {
        format!("::::: Credential Info :::::\n[c] Title: {}\n[c] Active: {}\n[c] User: {}\n[c] Password: {}\n",
            self.title,
            self.active,
            self.user,
            self.password)
    }
}
