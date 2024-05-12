use std::{collections::HashMap, fmt};

use crate::locales::LOCALES;

use chrono::FixedOffset;

/// Prints an offset from UTC in the format of `+HHMM` or `+HH:MM`.
/// `Z` instead of `+00[:]00` is allowed when `allow_zulu` is true.
pub fn write_local_minus_utc(w: &mut fmt::Formatter, off: FixedOffset, allow_zulu: bool, use_colon: bool) -> fmt::Result {
	let off = off.local_minus_utc();
	if !allow_zulu || off != 0 {
		let (sign, off) = if off < 0 { ('-', -off) } else { ('+', off) };
		if use_colon {
			write!(w, "{}{:02}:{:02}", sign, off / 3600, off / 60 % 60)
		} else {
			write!(w, "{}{:02}{:02}", sign, off / 3600, off / 60 % 60)
		}
	} else {
		write!(w, "Z")
	}
}

pub fn short_month(key: usize, locale: &str) -> &'static str {
	find_key(key, &LOCALES.short_months, locale).expect("Internal error: missing short months in the C locale")
}

pub fn long_month(key: usize, locale: &str) -> &'static str {
	find_key(key, &LOCALES.long_months, locale).expect("Internal error: missing long months in the C locale")
}

pub fn short_weekday(key: usize, locale: &str) -> &'static str {
	find_key(key, &LOCALES.short_weekdays, locale).expect("Internal error: missing short weekdays in the C locale")
}

pub fn long_weekday(key: usize, locale: &str) -> &'static str {
	find_key(key, &LOCALES.long_weekdays, locale).expect("Internal error: missing long weekdays in the C locale")
}

pub fn ampm(key: usize, locale: &str) -> &'static str {
	find_key(key, &LOCALES.ampm, locale).expect("Internal error: missing AM/PM in the C locale")
}

pub fn find_key(key: usize, data: &'static HashMap<String, Vec<&'static str>>, locale: &str) -> Option<&'static &'static str> {
	data.get(locale)
		.and_then(|res| res.get(key))
		.or_else(|| {
			if locale.contains('-') {
				locale
					.split('-')
					.collect::<Vec<&str>>()
					.first()
					.cloned()
					.and_then(|locale| data.get(locale).and_then(|res| res.get(key)))
			} else {
				None
			}
		})
		.or_else(|| data.get("C").and_then(|res| res.get(key)))
}
