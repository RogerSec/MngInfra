mod date; //my own implementation of ISO 8601 dates
use date::Date;

mod asset;
use asset::Asset;

//mod software;
//use software::Software;

mod credential;
use credential::Credential;


fn main() {
	println!("\n\n\n
  ,__ __                   _          _              
 /|  |  |                 | |        | |             
  |  |  |   _  _    __,   | | _  _   | |  ,_    __,  
  |  |  |  / |/ |  /  | _ |/ / |/ |  |/  /  |  /  |  
  |  |  |_/  |  |_/\\_/|/\\_/\\/  |  |_/|__/   |_/\\_/|_/
                     /|              |\\              
                    \\|              |/           
");
	let a = Asset::new_cli().unwrap();
	println!("{}", a.info());


    let c = Credential::new_from_cli().unwrap();
	println!("{}", c.info());

}
