#[macro_export]

macro_rules! return_if_q {
	($in:expr) => {{
		if $in.trim() == "q" {
			print!("\nquitting.."); 
			return Err("User quit input");
	}}};
}
