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
