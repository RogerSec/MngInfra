#[macro_export]
macro_rules! return_if_q {
	($in:expr) => {{
		if $in.trim() == "q" {
			print!("\nquitting.."); 
			return Err("User quit input".to_string());
	}}};
}

#[macro_export]
macro_rules! return_if_q_or_blank {
	($in:expr) => {{
		if $in.trim() == "q" {
			print!("\nquitting.."); 
			return Err("User quit input".to_string());
		}
		else if $in.trim() == "" {
			print!("\ncancelling.."); 
			return Err("User input blank".to_string());
		}
	}};
}

#[macro_export]
macro_rules! read_line {
	($in:ident) => {{
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut $in).expect("Error reading input");
	}};
}

#[macro_export]
macro_rules! read_line_Yn {
	($in:ident, $bool:ident) => {{
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut $in).expect("Error reading input");
		//if $in.trim().contains('')
	}};
}



