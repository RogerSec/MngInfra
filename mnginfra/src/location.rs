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
	pub fn new() -> Result<Self, String> {
		
	}
}
