extern crate chrono;
extern crate chrono_locale;

use chrono::{FixedOffset, TimeZone, Timelike};
use chrono_locale::LocaleDate;

// This test is copied from chrono's, disabling unsupported features
#[test]
fn format_it() {
	let dt = FixedOffset::east(34200).ymd(2001, 7, 8).and_hms_nano(0, 34, 59, 1_026_490_708);

	// date specifiers
	assert_eq!(dt.formatl("%Y", "it").to_string(), "2001");
	assert_eq!(dt.formatl("%C", "it").to_string(), "20");
	assert_eq!(dt.formatl("%y", "it").to_string(), "01");
	assert_eq!(dt.formatl("%m", "it").to_string(), "07");
	assert_eq!(dt.formatl("%b", "it").to_string(), "Lug");
	assert_eq!(dt.formatl("%B", "it").to_string(), "Luglio");
	assert_eq!(dt.formatl("%h", "it").to_string(), "Lug");
	assert_eq!(dt.formatl("%d", "it").to_string(), "08");
	assert_eq!(dt.formatl("%e", "it").to_string(), " 8");
	assert_eq!(dt.formatl("%e", "it").to_string(), dt.formatl("%_d", "it").to_string());
	assert_eq!(dt.formatl("%a", "it").to_string(), "Dom");
	assert_eq!(dt.formatl("%A", "it").to_string(), "Domenica");
	assert_eq!(dt.formatl("%w", "it").to_string(), "0");
	assert_eq!(dt.formatl("%u", "it").to_string(), "7");
	assert_eq!(dt.formatl("%U", "it").to_string(), "28");
	assert_eq!(dt.formatl("%W", "it").to_string(), "27");
	assert_eq!(dt.formatl("%G", "it").to_string(), "2001");
	assert_eq!(dt.formatl("%g", "it").to_string(), "01");
	assert_eq!(dt.formatl("%V", "it").to_string(), "27");
	assert_eq!(dt.formatl("%j", "it").to_string(), "189");
	assert_eq!(dt.formatl("%D", "it").to_string(), "07/08/01");
	assert_eq!(dt.formatl("%x", "it").to_string(), "07/08/01");
	assert_eq!(dt.formatl("%F", "it").to_string(), "2001-07-08");
	assert_eq!(dt.formatl("%v", "it").to_string(), " 8-Lug-2001");

	// time specifiers
	assert_eq!(dt.formatl("%H", "it").to_string(), "00");
	assert_eq!(dt.formatl("%k", "it").to_string(), " 0");
	assert_eq!(dt.formatl("%k", "it").to_string(), dt.formatl("%_H", "it").to_string());
	assert_eq!(dt.formatl("%I", "it").to_string(), "12");
	assert_eq!(dt.formatl("%l", "it").to_string(), "12");
	assert_eq!(dt.formatl("%l", "it").to_string(), dt.formatl("%_I", "it").to_string());
	assert_eq!(dt.formatl("%P", "it").to_string(), "am");
	assert_eq!(dt.formatl("%p", "it").to_string(), "AM");
	assert_eq!(dt.formatl("%M", "it").to_string(), "34");
	assert_eq!(dt.formatl("%S", "it").to_string(), "60");
	assert_eq!(dt.formatl("%f", "it").to_string(), "026490708");
	assert_eq!(dt.formatl("%.f", "it").to_string(), ".026490708");
	assert_eq!(dt.with_nanosecond(1_026_490_000).unwrap().formatl("%.f", "it").to_string(), ".026490");
	assert_eq!(dt.formatl("%.3f", "it").to_string(), ".026");
	assert_eq!(dt.formatl("%.6f", "it").to_string(), ".026490");
	assert_eq!(dt.formatl("%.9f", "it").to_string(), ".026490708");
	// The following formats are not exposed by chrono and cannot be formatted
	//		assert_eq!(dt.formatl("%3f", "it").to_string(), "026");
	//		assert_eq!(dt.formatl("%6f", "it").to_string(), "026490");
	//		assert_eq!(dt.formatl("%9f", "it").to_string(), "026490708");
	assert_eq!(dt.formatl("%R", "it").to_string(), "00:34");
	assert_eq!(dt.formatl("%T", "it").to_string(), "00:34:60");
	assert_eq!(dt.formatl("%X", "it").to_string(), "00:34:60");
	assert_eq!(dt.formatl("%r", "it").to_string(), "12:34:60 AM");

	// time zone specifiers
	//assert_eq!(dt.formatl("%Z", "it").to_string(), "ACST");
	assert_eq!(dt.formatl("%z", "it").to_string(), "+0930");
	assert_eq!(dt.formatl("%:z", "it").to_string(), "+09:30");

	// date & time specifiers
	assert_eq!(dt.formatl("%c", "it").to_string(), "Dom Lug  8 00:34:60 2001");
	assert_eq!(dt.formatl("%+", "it").to_string(), "2001-07-08T00:34:60.026490708+09:30");
	assert_eq!(
		dt.with_nanosecond(1_026_490_000).unwrap().formatl("%+", "it").to_string(),
		"2001-07-08T00:34:60.026490+09:30"
	);
	assert_eq!(dt.formatl("%s", "it").to_string(), "994518299");

	// special specifiers
	assert_eq!(dt.formatl("%t", "it").to_string(), "\t");
	assert_eq!(dt.formatl("%n", "it").to_string(), "\n");
	assert_eq!(dt.formatl("%%", "it").to_string(), "%");
}
