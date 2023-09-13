use crate::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Locales {
	pub short_months: HashMap<String, Vec<&'static str>>,
	pub long_months: HashMap<String, Vec<&'static str>>,
	pub short_weekdays: HashMap<String, Vec<&'static str>>,
	pub long_weekdays: HashMap<String, Vec<&'static str>>,
	pub ampm: HashMap<String, Vec<&'static str>>,
}

#[rustfmt::skip]
include!(concat!(env!("OUT_DIR"), "/locales.rs"));
