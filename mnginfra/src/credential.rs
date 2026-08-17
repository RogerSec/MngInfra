//credential stores credential information for a given software in a given asset

//currently we will not implement the password part, 
//as security configurations will have to be taken into account

struct Credential{
	pub title: String,
	pub user: String,
	password: String,
	pub active: bool,
}


impl Credential {
	fn new(title: String, user: Option<String>, active: Option<bool>) -> Result<Self, String> {
		if title.len() = 0 {return Err("invalid credential title when creating CREDENTIAL structure")}
		Ok( Self {
			title,
			user.unwrap_or(""),
			active.unwrap_or(true),
		})
	}
}
