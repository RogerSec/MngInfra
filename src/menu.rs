

struct Menu_item{
	title: String,
	goto: fn()
}

impl Menu_item {
	pub fn new(title: String, goto: fn()) -> Self {
		Self {
			title,
			goto,
		}
	}
}


struct Menu{
	title: String,
	//vec de menu_items
	current_option: u8
}

impl Menu {
	pub fn new(title: String) {
		Self {
			title,
			//ved de menu items vazio,
			current_option=0,
		}
	}
}
