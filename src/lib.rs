#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate num_integer;

use std::fmt;

use chrono::format::{Fixed, Item, Numeric, Pad, StrftimeItems};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveTime, Offset, TimeZone, Timelike};
use num_integer::{div_floor, mod_floor};

mod locales;

pub trait LocaleDate {
	fn formatl<'a>(&self, fmt: &'a str, locale: &str) -> DelayedFormatL10n<StrftimeItems<'a>>;
}

impl LocaleDate for chrono::NaiveDate {
	fn formatl<'a>(&self, fmt: &'a str, locale: &str) -> DelayedFormatL10n<StrftimeItems<'a>> {
		DelayedFormatL10n::new(Some(*self), None, StrftimeItems::new(fmt), locale)
	}
}

impl<Tz: TimeZone> LocaleDate for chrono::Date<Tz> {
	fn formatl<'a>(&self, fmt: &'a str, locale: &str) -> DelayedFormatL10n<StrftimeItems<'a>> {
		let offset = self.offset().fix();
		DelayedFormatL10n::new_with_offset(Some(self.naive_local()), None, &offset, StrftimeItems::new(fmt), locale)
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

impl<'a, I: Iterator<Item = Item<'a>> + Clone> fmt::Display for DelayedFormatL10n<I> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

/// This function is nearly entirely copied from chrono's format()
/// internal formats (3, 6 and 9-digits nanoseconds) have been disabled due to lack of access to chrono internals
pub fn format_l10n<'a, I>(
	w: &mut fmt::Formatter,
	date: Option<&NaiveDate>,
	time: Option<&NaiveTime>,
	off: Option<&(String, FixedOffset)>,
	items: I,
	locale: &str,
) -> fmt::Result
where
	I: Iterator<Item = Item<'a>>,
{
	for item in items {
		match item {
			Item::Literal(s) | Item::Space(s) => write!(w, "{}", s)?,
			Item::OwnedLiteral(ref s) | Item::OwnedSpace(ref s) => write!(w, "{}", s)?,

			Item::Numeric(spec, pad) => {
				use self::Numeric::*;

				let week_from_sun = |d: &NaiveDate| (d.ordinal() as i32 - d.weekday().num_days_from_sunday() as i32 + 7) / 7;
				let week_from_mon = |d: &NaiveDate| (d.ordinal() as i32 - d.weekday().num_days_from_monday() as i32 + 7) / 7;

				let (width, v) = match spec {
					Year => (4, date.map(|d| i64::from(d.year()))),
					YearDiv100 => (2, date.map(|d| div_floor(i64::from(d.year()), 100))),
					YearMod100 => (2, date.map(|d| mod_floor(i64::from(d.year()), 100))),
					IsoYear => (4, date.map(|d| i64::from(d.iso_week().year()))),
					IsoYearDiv100 => (2, date.map(|d| div_floor(i64::from(d.iso_week().year()), 100))),
					IsoYearMod100 => (2, date.map(|d| mod_floor(i64::from(d.iso_week().year()), 100))),
					Month => (2, date.map(|d| i64::from(d.month()))),
					Day => (2, date.map(|d| i64::from(d.day()))),
					WeekFromSun => (2, date.map(|d| i64::from(week_from_sun(d)))),
					WeekFromMon => (2, date.map(|d| i64::from(week_from_mon(d)))),
					IsoWeek => (2, date.map(|d| i64::from(d.iso_week().week()))),
					NumDaysFromSun => (1, date.map(|d| i64::from(d.weekday().num_days_from_sunday()))),
					WeekdayFromMon => (1, date.map(|d| i64::from(d.weekday().number_from_monday()))),
					Ordinal => (3, date.map(|d| i64::from(d.ordinal()))),
					Hour => (2, time.map(|t| i64::from(t.hour()))),
					Hour12 => (2, time.map(|t| i64::from(t.hour12().1))),
					Minute => (2, time.map(|t| i64::from(t.minute()))),
					Second => (2, time.map(|t| i64::from(t.second() + t.nanosecond() / 1_000_000_000))),
					Nanosecond => (9, time.map(|t| i64::from(t.nanosecond() % 1_000_000_000))),
					Timestamp => (
						1,
						match (date, time, off) {
							(Some(d), Some(t), None) => Some(d.and_time(*t).timestamp()),
							(Some(d), Some(t), Some(&(_, off))) => Some((d.and_time(*t) - off).timestamp()),
							(_, _, _) => None,
						},
					),

					// for the future expansion
					Internal(_) => (1, None),
				};

				if let Some(v) = v {
					if (spec == Year || spec == IsoYear) && !(0 <= v && v < 10_000) {
						// non-four-digit years require an explicit sign as per ISO 8601
						match pad {
							Pad::None => write!(w, "{:+}", v)?,
							Pad::Zero => write!(w, "{:+01$}", v, width + 1)?,
							Pad::Space => write!(w, "{:+1$}", v, width + 1)?,
						}
					} else {
						match pad {
							Pad::None => write!(w, "{}", v)?,
							Pad::Zero => write!(w, "{:01$}", v, width)?,
							Pad::Space => write!(w, "{:1$}", v, width)?,
						}
					}
				} else {
					return Err(fmt::Error); // insufficient arguments for given format
				}
			}

			Item::Fixed(spec) => {
				use self::Fixed::*;

				/// Prints an offset from UTC in the format of `+HHMM` or `+HH:MM`.
				/// `Z` instead of `+00[:]00` is allowed when `allow_zulu` is true.
				fn write_local_minus_utc(w: &mut fmt::Formatter, off: FixedOffset, allow_zulu: bool, use_colon: bool) -> fmt::Result {
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

				let ret = match spec {
					ShortMonthName => date.map(|d| write!(w, "{}", short_month(d.month0() as usize, locale))),
					LongMonthName => date.map(|d| write!(w, "{}", long_month(d.month0() as usize, locale))),
					ShortWeekdayName => date.map(|d| write!(w, "{}", short_weekday(d.weekday().num_days_from_monday() as usize, locale))),
					LongWeekdayName => date.map(|d| write!(w, "{}", long_weekday(d.weekday().num_days_from_monday() as usize, locale))),
					LowerAmPm => time.map(|t| write!(w, "{}", ampm(t.hour12().0 as usize, locale))),
					UpperAmPm => time.map(|t| write!(w, "{}", ampm(t.hour12().0 as usize, locale).to_uppercase())),
					Nanosecond => time.map(|t| {
						let nano = t.nanosecond() % 1_000_000_000;
						if nano == 0 {
							Ok(())
						} else if nano % 1_000_000 == 0 {
							write!(w, ".{:03}", nano / 1_000_000)
						} else if nano % 1_000 == 0 {
							write!(w, ".{:06}", nano / 1_000)
						} else {
							write!(w, ".{:09}", nano)
						}
					}),
					Nanosecond3 => time.map(|t| {
						let nano = t.nanosecond() % 1_000_000_000;
						write!(w, ".{:03}", nano / 1_000_000)
					}),
					Nanosecond6 => time.map(|t| {
						let nano = t.nanosecond() % 1_000_000_000;
						write!(w, ".{:06}", nano / 1_000)
					}),
					Nanosecond9 => time.map(|t| {
						let nano = t.nanosecond() % 1_000_000_000;
						write!(w, ".{:09}", nano)
					}),
					Internal(_) => panic!("Internal is not supported"),
					TimezoneName => off.map(|&(ref name, _)| write!(w, "{}", *name)),
					TimezoneOffsetColon => off.map(|&(_, off)| write_local_minus_utc(w, off, false, true)),
					TimezoneOffsetColonZ => off.map(|&(_, off)| write_local_minus_utc(w, off, true, true)),
					TimezoneOffset => off.map(|&(_, off)| write_local_minus_utc(w, off, false, false)),
					TimezoneOffsetZ => off.map(|&(_, off)| write_local_minus_utc(w, off, true, false)),
					RFC2822 =>
					// same to `%a, %e %b %Y %H:%M:%S %z`
					{
						if let (Some(d), Some(t), Some(&(_, off))) = (date, time, off) {
							let sec = t.second() + t.nanosecond() / 1_000_000_000;
							write!(
								w,
								"{}, {:2} {} {:04} {:02}:{:02}:{:02} ",
								short_weekday(d.weekday().num_days_from_monday() as usize, locale),
								d.day(),
								short_month(d.month0() as usize, locale),
								d.year(),
								t.hour(),
								t.minute(),
								sec
							)?;
							Some(write_local_minus_utc(w, off, false, false))
						} else {
							None
						}
					}
					RFC3339 =>
					// same to `%Y-%m-%dT%H:%M:%S%.f%:z`
					{
						if let (Some(d), Some(t), Some(&(_, off))) = (date, time, off) {
							// reuse `Debug` impls which already print ISO 8601 format.
							// this is faster in this way.
							write!(w, "{:?}T{:?}", d, t)?;
							Some(write_local_minus_utc(w, off, false, true))
						} else {
							None
						}
					}
				};

				match ret {
					Some(ret) => ret?,
					None => return Err(fmt::Error), // insufficient arguments for given format
				}
			}

			Item::Error => return Err(fmt::Error),
		}
	}

	Ok(())
}

fn short_month(month: usize, locale: &str) -> String {
	locales::LOCALES
		.short_months
		.get(locale)
		.or_else(|| locales::LOCALES.short_months.get("C"))
		.and_then(|res| res.get(month).map(|v| v.to_string()))
		.unwrap_or_else(|| format!("{}", month))
}

fn long_month(month: usize, locale: &str) -> String {
	locales::LOCALES
		.long_months
		.get(locale)
		.or_else(|| locales::LOCALES.long_months.get("C"))
		.and_then(|res| res.get(month).map(|v| v.to_string()))
		.unwrap_or_else(|| format!("{}", month))
}

fn short_weekday(day: usize, locale: &str) -> String {
	locales::LOCALES
		.short_weekdays
		.get(locale)
		.or_else(|| locales::LOCALES.short_weekdays.get("C"))
		.and_then(|res| res.get(day).map(|v| v.to_string()))
		.unwrap_or_else(|| format!("{}", day))
}

fn long_weekday(day: usize, locale: &str) -> String {
	locales::LOCALES
		.long_weekdays
		.get(locale)
		.or_else(|| locales::LOCALES.long_weekdays.get("C"))
		.and_then(|res| res.get(day).map(|v| v.to_string()))
		.unwrap_or_else(|| format!("{}", day))
}

fn ampm(spec: usize, locale: &str) -> String {
	locales::LOCALES
		.ampm
		.get(locale)
		.or_else(|| locales::LOCALES.ampm.get("C"))
		.and_then(|res| res.get(spec).map(|v| v.to_string()))
		.unwrap_or_else(|| format!("{}", spec))
}
