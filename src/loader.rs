#[cfg(not(target_arch = "wasm32"))]
use std::fs;

#[cfg(target_arch = "wasm32")]
use stdweb::js;

use crate::game::{GameSettings, MapInfo};

#[cfg(not(target_arch = "wasm32"))]
pub fn load(
    settings_path: &str,
    map_path: &str,
    callback: Box<dyn Fn(GameSettings, MapInfo) -> ()>,
) {
    let settings_str =
        fs::read_to_string("static/".to_owned() + settings_path).expect("Could not read settings");
    let map_str =
        fs::read_to_string("static/".to_owned() + map_path).expect("Could not read settings");
    callback(
        toml::from_str(&settings_str).expect("Could not parse settings"),
        toml::from_str(&map_str).expect("Could not parse settings"),
    );
}

#[cfg(target_arch = "wasm32")]
pub fn load(settings_path: &str, callback: Box<dyn Fn(GameSettings) -> ()>) {
    let handle = move |text: String| {
        callback(toml::from_str(&text).expect("Could not parse settings"));
    };
    js! {
        let handle = @{handle};
        let xhr = new XMLHttpRequest();

        xhr.addEventListener("load", () => {
            handle(xhr.responseText);
            handle.drop();
        });

        xhr.open("GET", @{settings_path});
        xhr.send();
    }
}
