pub struct Location {
	code: String, //most orgs have location codes for short and fast relations
	zone: String,
	country: String,
	city: String,
	building: String,
	room: String,
	exact: String,
	active: bool,
}


impl Location {
	fn new(code: String, exact: String) -> Result<Self, String> {
        
	}


    pub fn info(&self) -> String {
		format!("\r::::::: Location Info  :::::::\r", self.year, self.month, self.day)
	}
}
