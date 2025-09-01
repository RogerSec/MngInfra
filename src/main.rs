//use std::io;
//use std::mem;
use inquire::Select;
use std::collections::HashMap;

//pub mod menu;


fn display_banner()
{
	println!("\x1b[93m================\n||\x1b[36mMANAGE INFRA\x1b[93m||\n================\x1b[0m");
}



fn show_asset_onboard(){}
fn show_asset_edit(){}
fn show_asset_list(){}
fn show_asset_offboard(){}

fn show_asset_menu(){
	let mut option_map: HashMap<&str, fn()> = HashMap::new();
	
	option_map.insert("On-Board Asset", show_asset_onboard);
	option_map.insert("Edit Asset", show_asset_edit);
	option_map.insert("Asset list and states", show_asset_list);
	option_map.insert("Off-Board Asset", show_asset_offboard);
	
	
	let ans = Select::new("---Asset Management---", option_map.keys().collect()).prompt();
	
	match ans {
	    Ok(choice) => option_map.get(choice).unwrap()(),
	    Err(_) => println!("Main Menu"),
	}

}

fn show_network_menu(){ 
	print!("network");
}

fn show_vulnerability_menu(){ 
	print!("vuln");
}


fn show_connect_menu(){
	print!("con");
}


fn show_main_menu() -> bool {
	
	let mut option_map: HashMap<&str, fn()> = HashMap::new();
	
	option_map.insert("Asset Management", show_asset_menu);
	option_map.insert("Network Maps", show_network_menu);
	option_map.insert("Vulnerability Reports", show_vulnerability_menu);
	option_map.insert("Connect to asset", show_connect_menu);
	
	

	let ans = Select::new("---Main Menu---", option_map.keys().collect()).prompt();

	match ans {
	    Ok(choice) => option_map.get(choice).unwrap()(),
	    Err(_) => return false,
	}

	return true

}

fn main() {

	loop{
		print!("{}[2J", 27 as char); //clears the screen (or at least sends that signal)
		display_banner();
		if !show_main_menu(){
			break;
		}
	}
}
