use std::io::ErrorKind


pub struct IPv4 {
	octet: [u8; 4],
	range: u8, 
}

impl IPv4 {
	pub fn new(ip: String) -> Option<Self> {
		None
		Self {
			
		}	
}



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
	status: String, //prob an enum
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

	pub fn draw(&self) {
		print!("{}[2J", 27 as char); //clears the screen (or at least sends that signal)
		
		//banner:
		println!("     \x1b[93m================\n   ==||\x1b[36mMANAGE INFRA\x1b[93m||==\n     ================\x1b[0m\nEvery machine 1 jump away\n\n\n");


		println!("{}", self.title);

		self.menu_items.iter().enumerate().for_each(|(index, item)| {
			if index as u8 == self.current_option {print!("\x1b[93m=> \x1b[0m");}
			else {print!("   ");}
			print!("\x1b[36m{}\x1b[0m\n", &item.title);
		});
	}

	pub fn move_down(&mut self) {
		if self.current_option+1 < self.menu_items.len() as u8 { self.current_option += 1; }
	}

	pub fn move_up(&mut self) {
		if self.current_option > 0 { self.current_option -= 1; }
	}
	
	pub fn select(&self) {
		(self.menu_items[self.current_option as usize].goto)();
	}

	pub fn display(&mut self) {
	    if let Err(error) = listen(|event: Event| {
	        match event.event_type {
	            EventType::KeyPress(key) => println!("Key pressed: {:?}", key),
	            EventType::KeyRelease(key) => println!("Key released: {:?}", key),
	            _ => { println!("something?");}
	        }
	    }) {
	        eprintln!("Error: {:?}", error);
	    }
	}
}
