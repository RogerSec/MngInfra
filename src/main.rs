use std::io;
//use std::mem;



fn display_banner()
{
	println!("\x1b[93m================\n||\x1b[36mMANAGE INFRA\x1b[93m||\n================\x1b[0m");
}


fn read_menu_option(max_option: u8) -> u8
{
	let max_attempts = 5;
	let mut option = String::new();

	for attempt in 1..=max_attempts {
		io::stdin()
			.read_line(&mut option)
			.expect("Failed to read line");

		let opt: u8 = option.trim().parse().unwrap_or(0);

		if opt <= max_option && opt > 0 {return opt;}

		println!("non listed option.");
		if attempt+1 < max_attempts {println!("try again");}
		
	}

	return 0;
}


fn show_main_menu()
{
	println!("Choose an option:");

	println!("1)Asset Management\n2)Network Maps\n3)Connect to");
	let option = read_menu_option(3);
	if option == 0 {return;}
}

fn main() 
{
	display_banner();
	show_main_menu();
	println!("thx come again!");
	return;
}
