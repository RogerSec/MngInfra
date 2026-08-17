//since no date without library, this will be our date format (no time)
//simple ISO 8601 (YYYY-MM-DD)

pub struct Date{ 
	year: u16,
	month: u8,
	day: u8,
}

impl Date {

	fn is_valid_date(year: u16, month: u8, day: u8) -> bool {
		if year > 9999 || month > 12 || day > 31 {return false;} //not valid for ISO 8601
		//TODO: extra validation for dates that cant exist (i.e. 2029-02-29 or 2020-03-31)
		return true;
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

	pub fn get(&self) -> String {
		format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
	}
}
