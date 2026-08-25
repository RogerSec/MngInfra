use std::io;

pub struct Configuration{
	name: String,
	desc: String,
}


impl Configuration {
    fn new(name: String, description: String) -> Result<Self, String> {

        if name == "" {return Err("failed to construct configuration -  must have a name".to_string());}

        Ok(Self {
            name,
            desc: description,
        })
    }


    pub fn new_from_cli() -> Result<Self, String> {
        let mut name = String::new();
        let mut desc  = String::new();

        loop {
            name.clear();
            desc.clear();
            println!("Please input a name for the configuration");
            println!("(q to quit, r to restart)");
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");
            if name.trim() == "q" {return Err("user input canceled".to_string());}
            if name.trim() == "r" {continue;}


            println!("Please input a description for the configuration");
            println!("(q to quit, r to restart)");
            io::stdin()
                .read_line(&mut desc)
                .expect("Failed to read line");
            if desc.trim() == "q" {return Err("user input canceled".to_string());}
            if desc.trim() == "r" {continue;}

            break;
        }

            return Self::new(name.clone().trim().to_string(), desc.clone().trim().to_string());
    }
}
