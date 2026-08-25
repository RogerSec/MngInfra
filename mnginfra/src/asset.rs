
#[path = "software.rs"]
mod software;
use software::Software;

pub struct Asset{
	code: String,
	name: String,
	aliases: Vec<String>,
	installed_softwares: Vec<Software>, //not bool, this should be type "Software" but not yet
                                        //implemented
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
			installed_softwares: Vec::<Software>::new()
		})
	}

	pub fn info(&self) -> String {
		format!("Asset Info:::\n:+: Code: {}\n:+: Name:{}\n:+: Deplyoed: {}\n:+: # of Aliases: {}\n:+: # of Installed Softwares: {}\n:::::::::::", 
			self.code, 
			self.name, 
			self.deployed, 
			self.aliases.len(), 
			self.installed_softwares.len())
	}

    pub fn add_software(&mut self, software: Software) -> bool{
        let prev_size = self.installed_softwares.len();
        self.installed_softwares.push(software);
        if prev_size == self.installed_softwares.len()-1 {return true} return false
    }
}
