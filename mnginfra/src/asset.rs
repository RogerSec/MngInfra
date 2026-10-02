use std::io::{self, Write}; //used in asking for input in *_cli implementations


#[path = "software.rs"]
mod software;
use software::Software;

#[path = "configuration.rs"]
mod configuration;
use configuration::Configuration;

#[macro_use]
#[path = "macros_cli_helpers.rs"]
mod macros_cli_helpers;


pub struct Asset{
	code: String,
	name: String,
	configurations: Vec<Configuration>,
	aliases: Vec<String>,
	installed_softwares: Vec<Software>,
	deployed: bool,
}

impl Asset {
	pub fn new(asset_code: String, asset_name: Option<String>, asset_deployed: Option<bool>) -> Result<Self, String> {
		if asset_code.len() == 0 { return Err("Invalid asset code detected when creating ASSET structure".to_string())}
		let name = asset_name.unwrap_or("".to_string());
		let deployed = asset_deployed.unwrap_or(false);
		Ok( Self {
			code: asset_code,
			name,
			deployed,
			aliases: Vec::<String>::new(),
			installed_softwares: Vec::<Software>::new(),
			configurations : Vec::<Configuration>::new()
		})
	}

	pub fn new_from_cli() -> Result<Self, String> {
		print!("\n:::-------------------------:::\n");
		print!("   NEW ASSET   \n");
		let mut code = String::new();
		let mut name = String::new();
		let mut deployed = String::new();

		let asset_deployed: bool = true;

		print!("(q to quit asset creation)\n");

		print!("\n[Required] Asset Code: ");
		read_line!(code);
		return_if_q_or_blank!(code);

		print!("\n[Required] Asset is deployed? (Y/n): ");
		read_line!(deployed);
		return_if_q!(deployed);


		print!("\n[Optional] Asset Name: ");
		read_line!(name);
		return_if_q!(name);

		let new_asset = Asset::new(code, Some(name), Some(asset_deployed));




		print!("\n:::-------------------------:::\n");
		return Err("Not built yet..".to_string());
	}



	pub fn info(&self) -> String {
		format!("Asset Info:::\n:+: Code: {}\n:+: Name:{}\n:+: Deplyoed: {}\n:+: # of Aliases: {}\n:+: # of Installed Softwares: {}\n:+: # of Attached Configurations: {}\n:::::::::::", 
			self.code, 
			self.name, 
			self.deployed, 
			self.aliases.len(), 
			self.installed_softwares.len(),
			self.configurations.len())
	}

    fn add_software(&mut self, software: Software) -> bool{
        let prev_size = self.installed_softwares.len();
        self.installed_softwares.push(software);
        if prev_size == self.installed_softwares.len()-1 {return true} return false
    }


	// TODO pub fn add_software_cli

}
