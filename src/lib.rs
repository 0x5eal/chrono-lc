//! This is an extension to chrono's date and time formatting to automatically translate
//! dates to a specified locale or language
//!
//! ## Usage
//!
//! Put this in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! chrono = "0.4"
//! chrono_lc = "0.1.6"
//! ```
//!
//! Then put this in your `lib.rs` or `main.rs`:
//!
//! ```rust
//! use chrono::prelude::*;
//! use chrono_lc::LocaleDate;
//! ```
//!
//! You can choose to import just parts of chrono instead of the whole prelude.
//! Please see ['chrono`'s documentation](https://docs.rs/chrono/).
//!
//! To format a chrono `Date` or `DateTime` object, you can use the `formatl` method:
//!
//! ```rust
//! # use chrono::prelude::*;
//! # use chrono_lc::LocaleDate;
//! #
//! let dt = FixedOffset::east_opt(34200)
//!        .unwrap()
//!        .with_ymd_and_hms(2001, 7, 8, 0, 34, 59)
//!        .unwrap()
//!        .with_nanosecond(1_026_490_708)
//!        .unwrap();
//!
//! println!("{}", dt.formatl("%c", "fr"));
//! ```
//!
//! All of [chrono's formatting placeholders](https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html)
//! work except for `%3f`, `%6f` and `%9f` (but `%.3f`, `%.6f` and `%.9f` work normally)
//!
//! ## Locale format
//!
//! The `formatl` method supports locales in different formats, based on ISO-639-1 and ISO-3166.
//! It accepts any format where they are separated by `-` or `_`:
//!
//! - en_GB
//! - it-IT
//! - fr-fr
//! - PT_br
//!
//! The translated string will be first searched in the complete locale definition, then in the fallbacks.
//! For example: by requesting `it_IT` it will first try in `it-it`, then in `it` and, if it still
//! doesn't find it, it will use the default: `C` (english)
//!

#[allow(unused_imports)]
pub(crate) use lazy_static::lazy_static;

pub(crate) mod fmt;
pub(crate) mod locales;
pub(crate) mod util;

pub use crate::fmt::format_l10n;

use chrono::{
	format::{Item, StrftimeItems},
	FixedOffset, NaiveDate, NaiveTime, Offset, TimeZone,
};

pub trait LocaleDate {
	fn formatl<'a>(&self, fmt: &'a str, locale: &str) -> DelayedFormatL10n<StrftimeItems<'a>>;
}

impl LocaleDate for chrono::NaiveDate {
	fn formatl<'a>(&self, fmt: &'a str, locale: &str) -> DelayedFormatL10n<StrftimeItems<'a>> {
		DelayedFormatL10n::new(Some(*self), None, StrftimeItems::new(fmt), locale)
	}
}

impl LocaleDate for chrono::NaiveDateTime {
	fn formatl<'a>(&self, fmt: &'a str, locale: &str) -> DelayedFormatL10n<StrftimeItems<'a>> {
		DelayedFormatL10n::new(Some(self.date()), Some(self.time()), StrftimeItems::new(fmt), locale)
	}
}

impl<Tz: TimeZone> LocaleDate for chrono::DateTime<Tz> {
	fn formatl<'a>(&self, fmt: &'a str, locale: &str) -> DelayedFormatL10n<StrftimeItems<'a>> {
		let local = self.naive_local();
		let offset = self.offset().fix();
		DelayedFormatL10n::new_with_offset(Some(local.date()), Some(local.time()), &offset, StrftimeItems::new(fmt), locale)
	}
}

/// A *temporary* object which can be used as an argument to `format!` or others.
/// This is normally constructed via `format` methods of each date and time type.
#[derive(Debug)]
pub struct DelayedFormatL10n<I> {
	/// The locale to format the date in
	locale: String,
	/// The date view, if any.
	date: Option<NaiveDate>,
	/// The time view, if any.
	time: Option<NaiveTime>,
	/// The name and local-to-UTC difference for the offset (timezone), if any.
	off: Option<(String, FixedOffset)>,
	/// An iterator returning formatting items.
	items: I,
}

impl<'a, I: Iterator<Item = Item<'a>> + Clone> DelayedFormatL10n<I> {
	/// Makes a new `DelayedFormatL10n` value out of local date and time.
	pub fn new(date: Option<NaiveDate>, time: Option<NaiveTime>, items: I, locale: &str) -> DelayedFormatL10n<I> {
		DelayedFormatL10n {
			date,
			time,
			off: None,
			items,
			locale: locale.to_owned(),
		}
	}

	/// Makes a new `DelayedFormatL10n` value out of local date and time and UTC offset.
	pub fn new_with_offset(date: Option<NaiveDate>, time: Option<NaiveTime>, offset: &FixedOffset, items: I, locale: &str) -> DelayedFormatL10n<I> {
		let name_and_diff = (offset.to_string(), offset.to_owned());
		DelayedFormatL10n {
			date,
			time,
			off: Some(name_and_diff),
			items,
			locale: locale.to_owned(),
		}
	}
}

impl<'a, I: Iterator<Item = Item<'a>> + Clone> std::fmt::Display for DelayedFormatL10n<I> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		format_l10n(
			f,
			self.date.as_ref(),
			self.time.as_ref(),
			self.off.as_ref(),
			self.items.clone(),
			&self.locale,
		)
	}
}
