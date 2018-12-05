use std::collections::HashMap;

#[derive(Debug)]
pub struct Locales {
	pub short_months: HashMap<String, Vec<&'static str>>,
	pub long_months: HashMap<String, Vec<&'static str>>,
	pub short_weekdays: HashMap<String, Vec<&'static str>>,
	pub long_weekdays: HashMap<String, Vec<&'static str>>,
}

include!(concat!(env!("OUT_DIR"), "/locales.rs"));

