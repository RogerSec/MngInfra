//since no date without library, this will be our date format (no time)
//simple ISO 8601 (YYYY-MM-DD)

//----- WARNING: oldest supported date: 1179-01-01

use std::io;

pub struct Date{ 
	year: u16,
	month: u8,
	day: u8,
}

impl Date {

    fn is_leap_year(year: u16) -> bool {
        if year % 400 == 0 {return true;}

        if year % 100 == 0 {return false;}

        if year % 4 == 0 {return true;}

        return false;
    }

	fn is_valid_date(year: u16, month: u8, day: u8) -> bool {
		if year > 9999 || month > 12 || day > 31 {return false;} //not valid for ISO 8601
		//TODO: extra validation for dates that cant exist (i.e. 2029-02-29 or 2020-03-31)

        if year < 1179 {return false;} //not supported

        if month ==  1 && day > 31 {return false;}
        if month ==  2 && day > 29 {return false;}
        if month ==  3 && day > 31 {return false;}
        if month ==  4 && day > 30 {return false;}
        if month ==  5 && day > 31 {return false;}
        if month ==  6 && day > 30 {return false;}
        if month ==  7 && day > 30 {return false;}
        if month ==  8 && day > 31 {return false;}
        if month ==  9 && day > 30 {return false;}
        if month == 10 && day > 31 {return false;}
        if month == 11 && day > 30 {return false;}
        if month == 12 && day > 31 {return false;}


        if month == 2 && day == 29 && !Self::is_leap_year(year) {return false;}

        return true;
	}


    fn new_from_string(date_string: String) -> Result<Self, String> {
        //only for YYYYMMDD

        if !date_string.chars().all(|c| ('0'..='9').contains(&c)) { return Err(format!("date must be all digits (0 to 9): {}", date_string)); }

        if date_string.chars().count() != 8 {return Err(format!("date must be 8 characters long (YYYYMMDD): {}", date_string));}

        let mut year: u16 = 0;
        match date_string[0..4].parse::<u16>() {
            Ok(n) => year = n,
            Err(_e) => return Err("Error in YYYY".to_string()),
        }

        let mut month: u8 = 0;
        match date_string[4..6].parse::<u8>() {
            Ok(n) => month = n,
            Err(_e) => return Err("Error in MM".to_string()),
        }

        let mut day: u8 = 0;
        match date_string[6..8].parse::<u8>() {
            Ok(n) => day = n,
            Err(_e) => return Err("Error in DD".to_string()),
        }


        return Self::new(year, month, day);
    }


	pub fn new(year: u16, month: u8, day: u8) -> Result<Self, String> {
		//quick error checking
		if !Self::is_valid_date(year, month, day) {
			return Err(format!("invalid date detected when creating DATE structure: {}/{}/{}", 
				year, month, day))
		}
		Ok(Self {
			year,
			month,
			day
		})
	}

    pub fn new_from_cli() -> Result<Self, String> {
        let mut date = String::new();
        loop {
            println!("Please input a valid date in the following format: YYYYMMDD, i.e 20200522");
            println!("q to quit");
            io::stdin()
                .read_line(&mut date)
                .expect("Failed to read line");
            if date.contains("q") {return Err("user input canceled".to_string());}

            match Self::new_from_string(date.clone().trim().to_string()) {
                Ok(d) => return Ok(d),
                Err(e) => println!("[E:] {}", e),
            }

            date.clear();

        }
        //Ok()

    }


	pub fn info(&self) -> String {
		format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
	}
}
