extern crate walkdir;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use walkdir::WalkDir;

pub struct Locale {
	short_months: Vec<String>,
	long_months: Vec<String>,
	short_weekdays: Vec<String>,
	long_weekdays: Vec<String>,
}


fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("locales.rs");
	let mut f = File::create(&dest_path).unwrap();

	f.write_all(r###"
		use std::collections::HashMap;

		pub type Locales = HashMap<String, Locale>;

		pub struct Locale {
			short_months: Vec<String>,
			long_months: Vec<String>,
			short_weekdays: Vec<String>,
			long_weekdays: Vec<String>,
		}

		lazy_static! {
			static ref LOCALES: Locales = {
				let mut res = HashMap::new();
"###.as_bytes()).unwrap();


	println!("Building...");
	for entry in WalkDir::new("locales") {
		let entry = entry.unwrap();
		println!("{}", entry.path().display());

		if entry.path().extension().map(|e| e != "json").unwrap_or(false) {
			println!("Not a json file");
			continue
		}



	}


	f.write_all(r###"

				res
			};
		}
    "###.as_bytes()).unwrap();
}
