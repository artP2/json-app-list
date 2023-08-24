use freedesktop_desktop_entry as fde;
use fuzzy_filter;
use json;
use std::{env, fs};

fn main() {
    let mut json_app_list = json::JsonValue::new_array();
    for path in fde::Iter::new(fde::default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = fde::DesktopEntry::decode(&path, &bytes) {
                if let Some(name) = entry.name(None) {
                    if let Some(exec) = entry.exec() {
                        let app = json::object! {name: name.to_string(), exec: exec.to_string()};
                        let _ = json_app_list.push(app);
                    }
                }
            }
        }
    }
    let mut args = env::args();
    args.next(); // skip program name

    if let Some(filter) = args.next() {
        let filter_lower = filter.to_lowercase();
        let filtered_apps_iter = json_app_list.members().filter_map(|a| {
            let name_lower = a["name"].to_string().to_lowercase();
            if fuzzy_filter::matches(&filter_lower, name_lower.as_str()) {
                Some(a.to_owned())
            } else {
                None
            }
        });
        let mut json_app_list = json::JsonValue::new_array();
        filtered_apps_iter.for_each(|a| json_app_list.push(a.to_owned()).unwrap());

        println!("{}", json_app_list);
    } else {
        println!("{}", json_app_list);
    }
}
