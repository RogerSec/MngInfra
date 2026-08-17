mod date; //my own implementation of ISO 8601 dates
use date::Date;

mod asset;
use asset::Asset;



fn main() {
	println!("Hello!");
	let year = 2932;
	let mon = 8;
	let day = 19;

	println!("Creating new date: {}/{}/{}", year, mon, day);

	let d = Date::new(year, mon, day).unwrap();


	println!("{}", d.get());
	println!("Hello!");
	println!("Hello!");

	let asset_code = "SW_MNG_1".to_string();
	let asset_name = "switch da sala".to_string();

	println!("Creating new Asset");

	let a = Asset::new(asset_code, Some(asset_name), Some(true)).unwrap();

	println!("{}", a.info());






}



