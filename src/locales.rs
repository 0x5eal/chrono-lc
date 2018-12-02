// This file will be overwritten by build.rs during compilation

use std::collections::HashMap;

pub type Locales = HashMap<String, Locale>;

pub struct Locale {
	short_months: Vec<String>,
	long_months: Vec<String>,
	short_weekdays: Vec<String>,
	long_weekdays: Vec<String>,
}

lazy_static! {
	static ref LOCALES: Locales = {
		let mut res = HashMap::new();
		res.insert(
			"en".into(),
			Locale {
				short_months: vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
					.into_iter()
					.map(|s| s.to_owned())
					.collect::<Vec<String>>(),
				long_months: vec![
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
				].into_iter()
				.map(|s| s.to_owned())
				.collect::<Vec<String>>(),
				short_weekdays: vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
					.into_iter()
					.map(|s| s.to_owned())
					.collect::<Vec<String>>(),
				long_weekdays: vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
					.into_iter()
					.map(|s| s.to_owned())
					.collect::<Vec<String>>(),
			},
		);

		res
	};
}
