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
pub fn load(
    settings_path: &str,
    map_path: &str,
    callback: Box<dyn Fn(GameSettings, MapInfo) -> ()>,
) {
    let handle = move |s_text: String, m_text: String| {
        callback(
            toml::from_str(&s_text).expect("Could not parse settings"),
            toml::from_str(&m_text).expect("Could not parse map data"),
        );
    };
    js! {
        let handle = @{handle};

        let settings = undefined;
        let rsettings = new XMLHttpRequest();
        rsettings.open("GET", @{settings_path});
        rsettings.addEventListener("load", () => {
            settings = rsettings.responseText;
            if (map) {
                handle(settings, map);
                handle.drop();
            }
        });

        let map = undefined;
        let rmap = new XMLHttpRequest();
        rmap.open("GET", @{map_path});
        rmap.addEventListener("load", () => {
            map = rmap.responseText;
            if (settings) {
                handle(settings, map);
                handle.drop();
            }
        });

        rsettings.send();
        rmap.send();
    }
}
