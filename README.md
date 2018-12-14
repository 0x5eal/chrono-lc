# chrono-locale

This crate allows to format [chrono](https://github.com/chronotope/chrono) dates with localized months and week days.

It's in early development and everything could change. Use with caution!

## Usage

Put this in your Cargo.toml:

```
[dependencies]
chrono = "0.4"
chrono_locale = "0.1"
```

Then put this in your `lib.rs` or `main.rs`:

```
extern crate chrono;
extern crate chrono_locale;

use chrono::prelude::*;
use chrono_locale::LocaleDate;
```

You can choose to import just parts of chrono instead of the whole prelude.
Please see ['chrono`'s documentation](https://docs.rs/chrono/).

To format a chrono `Date` or `DateTime` object, you can use the `formatl` method:

```
let dt = FixedOffset::east(34200).ymd(2001, 7, 8).and_hms_nano(0, 34, 59, 1_026_490_708);
println!("{}", dt.formatl("%c", "fr"));
```

All of [chrono's formatting placeholders](https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html)
work except for `%3f`, `%6f` and `%9f` (but `%.3f`, `%.6f` and `%.9f` work normally)
