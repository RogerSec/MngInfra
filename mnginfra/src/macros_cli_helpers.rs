
#[macro_export]
macro_rules! clear_term {
	() => {{
	
    	print!("\x1B[2J\x1B[1;1H");
    	io::stdout().flush().unwrap();
	}};
}




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


#[macro_export]
macro_rules! display_title {
	($in:expr) => {{
		io::stdout().flush().unwrap();
		let spacers = $in.chars().count() + 16 + 4;
		print!("\n{}\n", "#".repeat(spacers));
		print!("##{}{}{}##\n",
			" ".repeat(8),
			$in,
			" ".repeat(8));
		print!("{}\n", "#".repeat(spacers));
		io::stdout().flush().unwrap();
	}};
}


