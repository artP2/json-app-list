use freedesktop_desktop_entry as fde;
use json;
use std::fs;

fn main() {
    let mut app_list = json::JsonValue::new_array();
    for path in fde::Iter::new(fde::default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = fde::DesktopEntry::decode(&path, &bytes) {
                if let Some(name) = entry.name(None) {
                    if let Some(exec) = entry.exec() {
                        let app = json::object! {
                            name: name.to_string(),
                            exec: exec.to_string()
                        };
                        let _ = app_list.push(app);
                    }
                }
            }
        }
    }
    println!("{}", app_list);
}
