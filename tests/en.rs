extern crate chrono;
extern crate chrono_locale;

use chrono::{FixedOffset, TimeZone, Timelike};
use chrono_locale::LocaleDate;

// This test is copied from chrono's, disabling unsupported features
#[test]
fn format_en() {
	let dt = FixedOffset::east(34200).ymd(2001, 7, 8).and_hms_nano(0, 34, 59, 1_026_490_708);

	// date specifiers
	assert_eq!(dt.formatl("%Y", "en").to_string(), "2001");
	assert_eq!(dt.formatl("%C", "en").to_string(), "20");
	assert_eq!(dt.formatl("%y", "en").to_string(), "01");
	assert_eq!(dt.formatl("%m", "en").to_string(), "07");
	assert_eq!(dt.formatl("%b", "en").to_string(), "Jul");
	assert_eq!(dt.formatl("%B", "en").to_string(), "July");
	assert_eq!(dt.formatl("%h", "en").to_string(), "Jul");
	assert_eq!(dt.formatl("%d", "en").to_string(), "08");
	assert_eq!(dt.formatl("%e", "en").to_string(), " 8");
	assert_eq!(dt.formatl("%e", "en").to_string(), dt.formatl("%_d", "en").to_string());
	assert_eq!(dt.formatl("%a", "en").to_string(), "Sun");
	assert_eq!(dt.formatl("%A", "en").to_string(), "Sunday");
	assert_eq!(dt.formatl("%w", "en").to_string(), "0");
	assert_eq!(dt.formatl("%u", "en").to_string(), "7");
	assert_eq!(dt.formatl("%U", "en").to_string(), "28");
	assert_eq!(dt.formatl("%W", "en").to_string(), "27");
	assert_eq!(dt.formatl("%G", "en").to_string(), "2001");
	assert_eq!(dt.formatl("%g", "en").to_string(), "01");
	assert_eq!(dt.formatl("%V", "en").to_string(), "27");
	assert_eq!(dt.formatl("%j", "en").to_string(), "189");
	assert_eq!(dt.formatl("%D", "en").to_string(), "07/08/01");
	assert_eq!(dt.formatl("%x", "en").to_string(), "07/08/01");
	assert_eq!(dt.formatl("%F", "en").to_string(), "2001-07-08");
	assert_eq!(dt.formatl("%v", "en").to_string(), " 8-Jul-2001");

	// time specifiers
	assert_eq!(dt.formatl("%H", "en").to_string(), "00");
	assert_eq!(dt.formatl("%k", "en").to_string(), " 0");
	assert_eq!(dt.formatl("%k", "en").to_string(), dt.formatl("%_H", "en").to_string());
	assert_eq!(dt.formatl("%I", "en").to_string(), "12");
	assert_eq!(dt.formatl("%l", "en").to_string(), "12");
	assert_eq!(dt.formatl("%l", "en").to_string(), dt.formatl("%_I", "en").to_string());
	assert_eq!(dt.formatl("%P", "en").to_string(), "am");
	assert_eq!(dt.formatl("%p", "en").to_string(), "AM");
	assert_eq!(dt.formatl("%M", "en").to_string(), "34");
	assert_eq!(dt.formatl("%S", "en").to_string(), "60");
	assert_eq!(dt.formatl("%f", "en").to_string(), "026490708");
	assert_eq!(dt.formatl("%.f", "en").to_string(), ".026490708");
	assert_eq!(dt.with_nanosecond(1_026_490_000).unwrap().formatl("%.f", "en").to_string(), ".026490");
	assert_eq!(dt.formatl("%.3f", "en").to_string(), ".026");
	assert_eq!(dt.formatl("%.6f", "en").to_string(), ".026490");
	assert_eq!(dt.formatl("%.9f", "en").to_string(), ".026490708");
	// The following formats are not exposed by chrono and cannot be formatted
	//		assert_eq!(dt.formatl("%3f", "en").to_string(), "026");
	//		assert_eq!(dt.formatl("%6f", "en").to_string(), "026490");
	//		assert_eq!(dt.formatl("%9f", "en").to_string(), "026490708");
	assert_eq!(dt.formatl("%R", "en").to_string(), "00:34");
	assert_eq!(dt.formatl("%T", "en").to_string(), "00:34:60");
	assert_eq!(dt.formatl("%X", "en").to_string(), "00:34:60");
	assert_eq!(dt.formatl("%r", "en").to_string(), "12:34:60 AM");

	// time zone specifiers
	//assert_eq!(dt.formatl("%Z", "en").to_string(), "ACST");
	assert_eq!(dt.formatl("%z", "en").to_string(), "+0930");
	assert_eq!(dt.formatl("%:z", "en").to_string(), "+09:30");

	// date & time specifiers
	assert_eq!(dt.formatl("%c", "en").to_string(), "Sun Jul  8 00:34:60 2001");
	assert_eq!(dt.formatl("%+", "en").to_string(), "2001-07-08T00:34:60.026490708+09:30");
	assert_eq!(
		dt.with_nanosecond(1_026_490_000).unwrap().formatl("%+", "en").to_string(),
		"2001-07-08T00:34:60.026490+09:30"
	);
	assert_eq!(dt.formatl("%s", "en").to_string(), "994518299");

	// special specifiers
	assert_eq!(dt.formatl("%t", "en").to_string(), "\t");
	assert_eq!(dt.formatl("%n", "en").to_string(), "\n");
	assert_eq!(dt.formatl("%%", "en").to_string(), "%");
}
