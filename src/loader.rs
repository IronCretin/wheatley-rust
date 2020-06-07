use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(target_arch = "wasm32")]
use stdweb::js;

use crate::game::GameSettings;
use crate::map::MapInfo;
use crate::monster::MonsterInfo;

#[cfg(not(target_arch = "wasm32"))]
pub fn load(
    settings_path: &str,
    map_path: &str,
    monsters_path: &str,
    callback: Box<dyn Fn(GameSettings, MapInfo, HashMap<String, MonsterInfo>) -> ()>,
) {
    let settings_str =
        fs::read_to_string("static/".to_owned() + settings_path).expect("Could not read settings");
    let map_str =
        fs::read_to_string("static/".to_owned() + map_path).expect("Could not read map info");
    let monsters_str =
        fs::read_to_string("static/".to_owned() + monsters_path).expect("Could not read monsters");
    callback(
        toml::from_str(&settings_str).expect("Could not parse settings"),
        toml::from_str(&map_str).expect("Could not parse map info"),
        toml::from_str(&monsters_str).expect("Could not parse monsters"),
    );
}

#[cfg(target_arch = "wasm32")]
pub fn load(
    settings_path: &str,
    map_path: &str,
    monsters_path: &str,
    callback: Box<dyn Fn(GameSettings, MapInfo, HashMap<String, MonsterInfo>) -> ()>,
) {
    let handle = move |settings_str: String, map_str: String, monsters_str: String| {
        callback(
            toml::from_str(&settings_str).expect("Could not parse settings"),
            toml::from_str(&map_str).expect("Could not parse map data"),
            toml::from_str(&monsters_str).expect("Could not parse monsters"),
        );
    };
    js! {
        let handle = @{handle};

        let xhrs = [];
        let resps = {};

        function add_req(path) {
            let xhr = new XMLHttpRequest();
            xhr.open("GET", path);
            xhr.addEventListener("load", () => {
                resps[path] = xhr.responseText;
                if (xhrs.every(x => x.readyState == 4)) {
                    handle(
                        resps[@{settings_path}],
                        resps[@{map_path}],
                        resps[@{monsters_path}]
                    );
                    handle.drop();
                }
            });
            xhrs.push(xhr);
        }

        add_req(@{settings_path});
        add_req(@{map_path});
        add_req(@{monsters_path});

        for (let x of xhrs) {
            x.send();
        }
    }
}
