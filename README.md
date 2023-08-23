# chrono-locale

This crate allows to format [chrono](https://github.com/chronotope/chrono) dates with localized months and week days. Backwards comptible fork of [Alex-PK/chrono-locale](https://github.com/Alex-PK/chrono-locale), now works with the latest version of chrono.

## Usage

Put this in your Cargo.toml:

```toml
[dependencies]
chrono = "0.4.56"
chrono_locale = { git = "https://github.com/0x5eal/chrono-locale.git", rev = "2a7ebcc" }
```

Then put this in your `lib.rs` or `main.rs`:

```rs
use chrono::prelude::*;
use chrono_locale::LocaleDate;
```

You can choose to import just parts of chrono instead of the whole prelude.
Please see ['chrono`'s documentation](https://docs.rs/chrono/).

To format a chrono `Date` or `DateTime` object, you can use the `formatl` method:

```rs
let dt = FixedOffset::east_opt(34200)
	.unwrap()
	.with_ymd_and_hms(2001, 7, 8, 0, 34, 59)
	.unwrap()
	.with_nanosecond(1_026_490_708)
	.unwrap();

println!("{}", dt.formatl("%c", "fr"));
```

All of [chrono's formatting placeholders](https://docs.rs/chrono/0.4.56/chrono/format/strftime/index.html)
work except for `%3f`, `%6f` and `%9f` (but `%.3f`, `%.6f` and `%.9f` work normally)
