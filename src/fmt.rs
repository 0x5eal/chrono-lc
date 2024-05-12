use std::fmt;

use crate::util;
use chrono::{
	format::{Fixed, Item, Numeric, Pad},
	Datelike, FixedOffset, NaiveDate, NaiveTime, Timelike,
};
use num_integer::{div_floor, mod_floor};

/// Parses a [Fixed] value and formats it.
pub fn parse_fixed(
	w: &mut fmt::Formatter,
	date: Option<&NaiveDate>,
	time: Option<&NaiveTime>,
	off: Option<&(String, FixedOffset)>,
	spec: &Fixed,
	locale: &str,
) -> Option<fmt::Result> {
	use self::Fixed::*;

	match spec {
		ShortMonthName => date.map(|d| write!(w, "{}", util::short_month(d.month0() as usize, locale))),
		LongMonthName => date.map(|d| write!(w, "{}", util::long_month(d.month0() as usize, locale))),
		ShortWeekdayName => date.map(|d| write!(w, "{}", util::short_weekday(d.weekday().num_days_from_monday() as usize, locale))),
		LongWeekdayName => date.map(|d| write!(w, "{}", util::long_weekday(d.weekday().num_days_from_monday() as usize, locale))),
		LowerAmPm => time.map(|t| write!(w, "{}", util::ampm(t.hour12().0 as usize, locale))),
		UpperAmPm => time.map(|t| write!(w, "{}", util::ampm(t.hour12().0 as usize + 2, locale))),
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
		TimezoneName => off.map(|(name, _)| write!(w, "{}", *name)),
		TimezoneOffsetColon => off.map(|&(_, off)| util::write_local_minus_utc(w, off, false, true)),
		TimezoneOffsetColonZ => off.map(|&(_, off)| util::write_local_minus_utc(w, off, true, true)),
		TimezoneOffsetDoubleColon => off.map(|&(_, off)| util::write_local_minus_utc(w, off, false, true)),
		TimezoneOffsetTripleColon => off.map(|&(_, off)| util::write_local_minus_utc(w, off, false, true)),
		TimezoneOffset => off.map(|&(_, off)| util::write_local_minus_utc(w, off, false, false)),
		TimezoneOffsetZ => off.map(|&(_, off)| util::write_local_minus_utc(w, off, true, false)),
		RFC2822 =>
		// same to `%a, %e %b %Y %H:%M:%S %z`
		{
			if let (Some(d), Some(t), Some(&(_, off))) = (date, time, off) {
				let sec = t.second() + t.nanosecond() / 1_000_000_000;
				write!(
					w,
					"{}, {:2} {} {:04} {:02}:{:02}:{:02} ",
					util::short_weekday(d.weekday().num_days_from_monday() as usize, locale),
					d.day(),
					util::short_month(d.month0() as usize, locale),
					d.year(),
					t.hour(),
					t.minute(),
					sec
				)
				.ok()?;
				Some(util::write_local_minus_utc(w, off, false, false))
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
				write!(w, "{:?}T{:?}", d, t).ok()?;
				Some(util::write_local_minus_utc(w, off, false, true))
			} else {
				None
			}
		}

		spec => todo!("Support for formatting fixed format {:?} is yet to be implemented!", spec),
	}
}

/// Parses a [Numeric] value and returns its width and its formattable component.
pub fn parse_numeric(
	date: Option<&NaiveDate>,
	time: Option<&NaiveTime>,
	off: Option<&(String, FixedOffset)>,
	spec: &Numeric,
) -> (usize, Option<i64>) {
	use self::Numeric::*;

	let week_from_sun = |d: &NaiveDate| (d.ordinal() as i32 - d.weekday().num_days_from_sunday() as i32 + 7) / 7;
	let week_from_mon = |d: &NaiveDate| (d.ordinal() as i32 - d.weekday().num_days_from_monday() as i32 + 7) / 7;

	match spec {
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
				(Some(d), Some(t), None) => Some(d.and_time(*t).and_utc().timestamp()),
				(Some(d), Some(t), Some(&(_, off))) => Some((d.and_time(*t) - off).and_utc().timestamp()),
				(_, _, _) => None,
			},
		),
		// for the future expansion
		Internal(_) => (1, None),
		spec => todo!("Support for formatting numeric format {:?} is yet to be implemented!", spec),
	}
}

/// This function is nearly entirely copied from chrono's format()
/// internal formats (3, 6 and 9-digits nanoseconds) have been disabled due to lack of access to chrono internals
pub fn format_l10n<'a, I>(
	w: &mut std::fmt::Formatter,
	date: Option<&NaiveDate>,
	time: Option<&NaiveTime>,
	off: Option<&(String, FixedOffset)>,
	items: I,
	locale: &str,
) -> std::fmt::Result
where
	I: Iterator<Item = Item<'a>>,
{
	let locale = locale.to_lowercase().replace('_', "-");
	for item in items {
		match item {
			Item::Literal(s) | Item::Space(s) => write!(w, "{}", s)?,
			Item::OwnedLiteral(ref s) | Item::OwnedSpace(ref s) => write!(w, "{}", s)?,

			Item::Numeric(spec, pad) => {
				use self::Numeric::{IsoYear, Year};
				let (width, v) = parse_numeric(date, time, off, &spec);

				if let Some(v) = v {
					if (spec == Year || spec == IsoYear) && !(0..10_000).contains(&v) {
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
					return Err(std::fmt::Error); // insufficient arguments for given format
				}
			}

			Item::Fixed(spec) => parse_fixed(w, date, time, off, &spec, &locale).ok_or(std::fmt::Error)??,
			Item::Error => return Err(std::fmt::Error),
		}
	}

	Ok(())
}
