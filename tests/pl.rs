use chrono::{FixedOffset, TimeZone, Timelike};
use chrono_lc::LocaleDate;

// This test is copied from chrono's, disabling unsupported features
#[test]
fn format_pl() {
	let dt = FixedOffset::east_opt(34200)
		.expect("out of bound")
		.with_ymd_and_hms(2001, 7, 8, 0, 34, 59)
		.unwrap()
		.with_nanosecond(1_026_490_708)
		.expect("out of bound");
	let locale = "pl";

	// date specifiers
	assert_eq!(dt.formatl("%Y", locale).to_string(), "2001");
	assert_eq!(dt.formatl("%C", locale).to_string(), "20");
	assert_eq!(dt.formatl("%y", locale).to_string(), "01");
	assert_eq!(dt.formatl("%m", locale).to_string(), "07");
	assert_eq!(dt.formatl("%b", locale).to_string(), "lip");
	assert_eq!(dt.formatl("%B", locale).to_string(), "lipiec");
	assert_eq!(dt.formatl("%h", locale).to_string(), "lip");
	assert_eq!(dt.formatl("%d", locale).to_string(), "08");
	assert_eq!(dt.formatl("%e", locale).to_string(), " 8");
	assert_eq!(dt.formatl("%e", locale).to_string(), dt.formatl("%_d", locale).to_string());
	assert_eq!(dt.formatl("%a", locale).to_string(), "niedz");
	assert_eq!(dt.formatl("%A", locale).to_string(), "niedziela");
	assert_eq!(dt.formatl("%w", locale).to_string(), "0");
	assert_eq!(dt.formatl("%u", locale).to_string(), "7");
	assert_eq!(dt.formatl("%U", locale).to_string(), "28");
	assert_eq!(dt.formatl("%W", locale).to_string(), "27");
	assert_eq!(dt.formatl("%G", locale).to_string(), "2001");
	assert_eq!(dt.formatl("%g", locale).to_string(), "01");
	assert_eq!(dt.formatl("%V", locale).to_string(), "27");
	assert_eq!(dt.formatl("%j", locale).to_string(), "189");
	assert_eq!(dt.formatl("%D", locale).to_string(), "07/08/01");
	assert_eq!(dt.formatl("%x", locale).to_string(), "07/08/01");
	assert_eq!(dt.formatl("%F", locale).to_string(), "2001-07-08");
	assert_eq!(dt.formatl("%v", locale).to_string(), " 8-lip-2001");

	// time specifiers
	assert_eq!(dt.formatl("%H", locale).to_string(), "00");
	assert_eq!(dt.formatl("%k", locale).to_string(), " 0");
	assert_eq!(dt.formatl("%k", locale).to_string(), dt.formatl("%_H", locale).to_string());
	assert_eq!(dt.formatl("%I", locale).to_string(), "12");
	assert_eq!(dt.formatl("%l", locale).to_string(), "12");
	assert_eq!(dt.formatl("%l", locale).to_string(), dt.formatl("%_I", locale).to_string());
	assert_eq!(dt.formatl("%P", locale).to_string(), "am");
	assert_eq!(dt.formatl("%p", locale).to_string(), "AM");
	assert_eq!(dt.formatl("%M", locale).to_string(), "34");
	assert_eq!(dt.formatl("%S", locale).to_string(), "60");
	assert_eq!(dt.formatl("%f", locale).to_string(), "026490708");
	assert_eq!(dt.formatl("%.f", locale).to_string(), ".026490708");
	assert_eq!(dt.with_nanosecond(1_026_490_000).unwrap().formatl("%.f", locale).to_string(), ".026490");
	assert_eq!(dt.formatl("%.3f", locale).to_string(), ".026");
	assert_eq!(dt.formatl("%.6f", locale).to_string(), ".026490");
	assert_eq!(dt.formatl("%.9f", locale).to_string(), ".026490708");
	// The following formats are not exposed by chrono and cannot be formatted
	//		assert_eq!(dt.formatl("%3f", locale).to_string(), "026");
	//		assert_eq!(dt.formatl("%6f", locale).to_string(), "026490");
	//		assert_eq!(dt.formatl("%9f", locale).to_string(), "026490708");
	assert_eq!(dt.formatl("%R", locale).to_string(), "00:34");
	assert_eq!(dt.formatl("%T", locale).to_string(), "00:34:60");
	assert_eq!(dt.formatl("%X", locale).to_string(), "00:34:60");
	assert_eq!(dt.formatl("%r", locale).to_string(), "12:34:60 AM");

	// time zone specifiers
	//assert_eq!(dt.formatl("%Z", locale).to_string(), "ACST");
	assert_eq!(dt.formatl("%z", locale).to_string(), "+0930");
	assert_eq!(dt.formatl("%:z", locale).to_string(), "+09:30");

	// date & time specifiers
	assert_eq!(dt.formatl("%c", locale).to_string(), "niedz lip  8 00:34:60 2001");
	assert_eq!(dt.formatl("%+", locale).to_string(), "2001-07-08T00:34:60.026490708+09:30");
	assert_eq!(
		dt.with_nanosecond(1_026_490_000).unwrap().formatl("%+", locale).to_string(),
		"2001-07-08T00:34:60.026490+09:30"
	);
	assert_eq!(dt.formatl("%s", locale).to_string(), "994518299");

	// special specifiers
	assert_eq!(dt.formatl("%t", locale).to_string(), "\t");
	assert_eq!(dt.formatl("%n", locale).to_string(), "\n");
	assert_eq!(dt.formatl("%%", locale).to_string(), "%");
}

#[test]
fn format_pl_naive() {
	let dt = FixedOffset::east_opt(34200)
		.expect("out of bound")
		.with_ymd_and_hms(2001, 7, 8, 0, 34, 59)
		.unwrap()
		.with_nanosecond(1_026_490_708)
		.expect("out of bound");
	let dt = dt.naive_local();
	let locale = "pl";

	// date specifiers
	assert_eq!(dt.formatl("%Y", locale).to_string(), "2001");
	assert_eq!(dt.formatl("%C", locale).to_string(), "20");
	assert_eq!(dt.formatl("%y", locale).to_string(), "01");
	assert_eq!(dt.formatl("%m", locale).to_string(), "07");
	assert_eq!(dt.formatl("%b", locale).to_string(), "lip");
	assert_eq!(dt.formatl("%B", locale).to_string(), "lipiec");
	assert_eq!(dt.formatl("%h", locale).to_string(), "lip");
	assert_eq!(dt.formatl("%d", locale).to_string(), "08");
	assert_eq!(dt.formatl("%e", locale).to_string(), " 8");
	assert_eq!(dt.formatl("%e", locale).to_string(), dt.formatl("%_d", locale).to_string());
	assert_eq!(dt.formatl("%a", locale).to_string(), "niedz");
	assert_eq!(dt.formatl("%A", locale).to_string(), "niedziela");
	assert_eq!(dt.formatl("%w", locale).to_string(), "0");
	assert_eq!(dt.formatl("%u", locale).to_string(), "7");
	assert_eq!(dt.formatl("%U", locale).to_string(), "28");
	assert_eq!(dt.formatl("%W", locale).to_string(), "27");
	assert_eq!(dt.formatl("%G", locale).to_string(), "2001");
	assert_eq!(dt.formatl("%g", locale).to_string(), "01");
	assert_eq!(dt.formatl("%V", locale).to_string(), "27");
	assert_eq!(dt.formatl("%j", locale).to_string(), "189");
	assert_eq!(dt.formatl("%D", locale).to_string(), "07/08/01");
	assert_eq!(dt.formatl("%x", locale).to_string(), "07/08/01");
	assert_eq!(dt.formatl("%F", locale).to_string(), "2001-07-08");
	assert_eq!(dt.formatl("%v", locale).to_string(), " 8-lip-2001");

	// time specifiers
	assert_eq!(dt.formatl("%H", locale).to_string(), "00");
	assert_eq!(dt.formatl("%k", locale).to_string(), " 0");
	assert_eq!(dt.formatl("%k", locale).to_string(), dt.formatl("%_H", locale).to_string());
	assert_eq!(dt.formatl("%I", locale).to_string(), "12");
	assert_eq!(dt.formatl("%l", locale).to_string(), "12");
	assert_eq!(dt.formatl("%l", locale).to_string(), dt.formatl("%_I", locale).to_string());
	assert_eq!(dt.formatl("%P", locale).to_string(), "am");
	assert_eq!(dt.formatl("%p", locale).to_string(), "AM");
	assert_eq!(dt.formatl("%M", locale).to_string(), "34");
	assert_eq!(dt.formatl("%S", locale).to_string(), "60");
	assert_eq!(dt.formatl("%R", locale).to_string(), "00:34");
	assert_eq!(dt.formatl("%T", locale).to_string(), "00:34:60");
	assert_eq!(dt.formatl("%X", locale).to_string(), "00:34:60");
	assert_eq!(dt.formatl("%r", locale).to_string(), "12:34:60 AM");

	// date & time specifiers
	assert_eq!(dt.formatl("%c", locale).to_string(), "niedz lip  8 00:34:60 2001");
	//	assert_eq!(dt.formatl("%+", locale).to_string(), "2001-07-08T00:34:60.026490708+09:30");
	assert_eq!(dt.formatl("%s", locale).to_string(), "994552499");

	// special specifiers
	assert_eq!(dt.formatl("%t", locale).to_string(), "\t");
	assert_eq!(dt.formatl("%n", locale).to_string(), "\n");
	assert_eq!(dt.formatl("%%", locale).to_string(), "%");
}
