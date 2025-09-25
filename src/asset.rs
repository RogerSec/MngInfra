
pub struct Asset {
	name: String,
	uid: String,
	type: String,
	ip: String, //horrible, TODO: change this to another data type more appropriate
	reachable: bool,
	alive_icmp: bool,
	location: String,
	notes: String,
	aliases: String,
	mac_address: String, //also horrible, TODO: get a better data-type
	status: String, //prob an enum? active/deployed/retired/backup
	vendor: String,
	short_code: String //this should also be a data-type - but if not green-field it would be horrible to force a short-code
}

impl Asset {
	pub fn new(name: String, menu_items: Vec<MenuItem>) -> Self {
		Self {
			title,
			menu_items,
			current_option: 0,
		}
	}
}
