extern crate walkdir;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{Write, Read, Error as IoError};
use std::path::Path;
use serde_json::{Error as JsonError};

use walkdir::{WalkDir, DirEntry};

#[derive(Deserialize)]
pub struct Locale {
	short_months: Option<Vec<String>>,
	long_months: Option<Vec<String>>,
	short_weekdays: Option<Vec<String>>,
	long_weekdays: Option<Vec<String>>,
}

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("locales.rs");
	let mut f = File::create(&dest_path).unwrap();

	let _ = f.write_all(r#####"lazy_static! {
		pub static ref LOCALES: Locales = {
			let mut res = Locales {
				short_months: HashMap::new(),
				long_months: HashMap::new(),
				short_weekdays: HashMap::new(),
				long_weekdays: HashMap::new(),
			};

			res.short_months.insert(
				"C".into(),
				vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
			);

			res.long_months.insert(
				"C".into(),
				vec![
					"January",
					"February",
					"March",
					"April",
					"May",
					"June",
					"July",
					"August",
					"September",
					"October",
					"November",
					"December",
				],
			);

			res.short_weekdays
				.insert("C".into(), vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]);

			res.long_weekdays.insert(
				"C".into(),
				vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"],
			);
	"#####.as_bytes());

	println!("Building...");
	for entry in WalkDir::new("locales") {
		let entry = entry.unwrap();
		println!("Found {}", entry.path().display());

		if entry.path().extension().map(|e| e != "json").unwrap_or(false) {
			println!("Not a json file");
			continue
		}

		let locale_name = entry.path().file_stem().map(|n| n.to_string_lossy());
		if locale_name.is_none() {
			continue;
		}


		let locale_name = locale_name.unwrap().to_string();
		if let Ok(locale_data) = load_locale(&entry) {
			if let Some(long_months) = locale_data.long_months {
				if long_months.len() == 12 {
					let _ = f.write_all(format!(
						"res.long_months.insert(\"{}\".into(), vec![{}]);\n",
						locale_name,
						long_months
							.iter()
							.map(|s| format!("\"{}\"", s))
							.collect::<Vec<String>>()
							.join(",")
					).as_bytes()).unwrap();
				}
			}

			if let Some(short_months) = locale_data.short_months {
				if short_months.len() == 12 {
					let _ = f.write_all(format!(
						"res.short_months.insert(\"{}\".into(), vec![{}]);\n",
						locale_name,
						short_months
							.iter()
							.map(|s| format!("\"{}\"", s))
							.collect::<Vec<String>>()
							.join(",")
					).as_bytes()).unwrap();
				}
			}

			if let Some(long_weekdays) = locale_data.long_weekdays {
				if long_weekdays.len() == 7 {
					let _ = f.write_all(format!(
						"res.long_weekdays.insert(\"{}\".into(), vec![{}]);\n",
						locale_name,
						long_weekdays
							.iter()
							.map(|s| format!("\"{}\"", s))
							.collect::<Vec<String>>()
							.join(",")
					).as_bytes()).unwrap();
				}
			}

			if let Some(short_weekdays) = locale_data.short_weekdays {
				if short_weekdays.len() == 7 {
					let _ = f.write_all(format!(
						"res.short_weekdays.insert(\"{}\".into(), vec![{}]);\n",
						locale_name,
						short_weekdays
							.iter()
							.map(|s| format!("\"{}\"", s))
							.collect::<Vec<String>>()
							.join(",")
					).as_bytes()).unwrap();
				}
			}
		}
	}

	let _ = f.write_all(r####"		res
			};
		}
		"####.as_bytes());
}

fn load_locale(entry: &DirEntry) -> Result<Locale, BuildError> {
	let mut locale_data = String::new();
	let mut f = File::open(entry.path())?;
	f.read_to_string(&mut locale_data)?;
	let locale = serde_json::from_str::<Locale>(&locale_data)?;
	Ok(locale)
}

enum BuildError {
	Io(IoError),
	Json(JsonError),
}

impl From<IoError> for BuildError {
	fn from(e: IoError) -> Self {
		BuildError::Io(e)
	}
}

impl From<JsonError> for BuildError {
	fn from(e: JsonError) -> Self {
		BuildError::Json(e)
	}
}
