//use std::io;
//use std::mem;


pub mod menu;


fn display_banner()
{
	println!("\x1b[93m================\n||\x1b[36mMANAGE INFRA\x1b[93m||\n================\x1b[0m");
}

//
//fn read_menu_option(max_option: u8) -> u8
//{
//	let max_attempts = 5;
//	let mut option = String::new();
//	let mut opt: u8;
//
//	for attempt in 1..=max_attempts {
//		option.clear();
//		io::stdin()
//			.read_line(&mut option)
//			.expect("Failed to read line");
//
//		opt = option.trim().parse().unwrap_or(0);
//
//		if opt <= max_option && opt > 0 {return opt;}
//
//		println!("non listed option.");
//		if attempt+1 < max_attempts {println!("try again");}
//	}
//
//	return 0;
//}

fn show_asset_menu(){}

fn show_network_menu(){ print!("aaaa");}

fn show_connect_menu(){}


fn show_main_menu()
{
	let mut main_menu = menu::Menu::new("----- Home -----".to_string(),
		vec![
			menu::MenuItem::new("Asset Management".to_string(),show_asset_menu),
			menu::MenuItem::new("Network Maps".to_string(),show_network_menu),
			menu::MenuItem::new("Connect to Machine".to_string(),show_connect_menu)
		]
	);


	main_menu.display();
//	main_menu.move_down();
//	main_menu.display();
//	main_menu.select();

//
//	println!("Choose an option:");
//
//	println!("1)Asset Management\n2)Network Maps\n3)Connect to");
//	let option = read_menu_option(3);
//	if option == 0 {return;}
//
//	println!("Selected Option: {}",option);
//	match option{
//		1=>show_asset_menu(),
//		2=>show_network_menu(),
//		3=>show_connect_menu(),
//	}
//
}

fn main() 
{
	display_banner();

	show_main_menu();
	return;
}
