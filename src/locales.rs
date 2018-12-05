// This file will be overwritten by build.rs during compilation

use std::collections::HashMap;

pub struct Locales {
	pub short_months: HashMap<String, Vec<&'static str>>,
	pub long_months: HashMap<String, Vec<&'static str>>,
	pub short_weekdays: HashMap<String, Vec<&'static str>>,
	pub long_weekdays: HashMap<String, Vec<&'static str>>,
}

lazy_static! {
	pub static ref LOCALES: Locales = {
		let mut res = Locales {
			short_months: HashMap::new(),
			long_months: HashMap::new(),
			short_weekdays: HashMap::new(),
			long_weekdays: HashMap::new(),
		};

		res.short_months.insert(
			"C".into(),
			vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
		);

		res.long_months.insert(
			"C".into(),
			vec![
				"January",
				"February",
				"March",
				"April",
				"May",
				"June",
				"July",
				"August",
				"September",
				"October",
				"November",
				"December",
			],
		);

		res.short_weekdays
			.insert("C".into(), vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]);

		res.long_weekdays.insert(
			"C".into(),
			vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"],
		);

		res
	};
}
