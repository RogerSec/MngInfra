#[path = "date.rs"]
mod date;
use date::Date;

#[path = "configuration.rs"]
mod configuration;
use configuration::Configuration;

#[path = "credential.rs"]
mod credential;
use credential::Credential;

pub struct Software{
	name: String,
	vendor: String,
	version: String,
	last_seen: Date,
	software_type: String,
	observations: String,
	configs: Vec<Configuration>,
	credentials: Vec<Credential>,
}

impl Software{
    pub fn new() -> Result<Self, String> {
        return Err("not built yet".to_string());
    }
}
