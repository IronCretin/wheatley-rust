use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::Hasher;
use std::rc::Rc;

// use rand::random;

#[cfg(target_arch = "wasm32")]
use stdweb::console;
#[cfg(not(target_arch = "wasm32"))]
macro_rules! console {
    (log, $arg:expr) => {
        println!("{:?}", $arg)
    };
    (error, $arg:expr) => {
        eprintln!("{:?}", $arg)
    };
}

use doryen_rs::{App, AppOptions};

pub mod colors;
pub mod game;
pub mod loader;
pub mod map;
pub mod player;
pub mod point;
pub mod screen;
pub mod tile;

use colors::{BLACK, DARK_GREEN};
use game::Game;
use loader::load;
use screen::game::GameScreen;
use screen::menu::MenuScreen;
use screen::textbox::TextBox;
use screen::{Action, WheatleyEngine};
use tile::Tile;

pub const PLAYER_TILE: Tile = Tile {
    ch: '@' as u16,
    fg: DARK_GREEN,
    bg: BLACK,
};

fn main() {
    if cfg!(target_arch = "wasm32") {
        std::panic::set_hook(Box::new(|p| {
            let s = p.to_string();
            console!(error, s)
        }));
    }

    load(
        "settings.toml",
        "map.toml",
        Box::new(|settings, map_info| {
            let mut app = App::new(AppOptions {
                console_width: settings.interface.width,
                console_height: settings.interface.height,
                screen_width: settings.interface.width * settings.interface.font.width,
                screen_height: settings.interface.height * settings.interface.font.height,
                window_title: "Wheatley Simulator".to_owned(),
                font_path: settings.interface.font.font.clone(),
                resizable: false,
                ..AppOptions::default()
            });

            let mut hasher = DefaultHasher::new();
            let seed: u64 = if let Some(_) = env::args().nth(1).map(|s| hasher.write(s.as_bytes()))
            {
                hasher.finish()
            } else {
                0
            };

            let help = Rc::new(TextBox::new(
                Some(String::from("Help")),
                String::from(
                    r#"? - Show help screen
    Arrow keys: move around
    You can also use numpad or vi-keys:
    7 8 9    y k u
    4 @ 6    h @ l
    1 2 3    b j n"#,
                ),
                50,
                30,
                true,
            ));

            let engine = WheatleyEngine::new(Game::new(
                settings, map_info,
                Rc::new(MenuScreen::new(String::from(
r#"+-------------------------------------------------------------------------+
|           __          ___                _   _                          |
|           \ \        / / |              | | | |                         |
|            \ \  /\  / /| |__   ___  __ _| |_| | ___ _   _               |
|             \ \/  \/ / | '_ \ / _ \/ _` | __| |/ _ \ | | |              |
|              \  /\  /  | | | |  __/ (_| | |_| |  __/ |_| |              |
|               \/  \/   |_| |_|\___|\__,_|\__|_|\___|\__, |              |
|                                                      __/ |              |
|                                                     |___/               |
|   _____ _                 _       _               ___   ___ ___   ___   |
|  / ____(_)               | |     | |             |__ \ / _ \__ \ / _ \  |
| | (___  _ _ __ ___  _   _| | __ _| |_ ___  _ __     ) | | | | ) | | | | |
|  \___ \| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|   / /| | | |/ /| | | | |
|  ____) | | | | | | | |_| | | (_| | || (_) | |     / /_| |_| / /_| |_| | |
| |_____/|_|_| |_| |_|\__,_|_|\__,_|\__\___/|_|    |____|\___/____|\___/  |
+-------------------------------------------------------------------------+"#),
                vec![
                    (String::from("Play!"), Action::Push(Rc::new(GameScreen::new()))),
                    (String::from("Help"), Action::Push(help.clone())),
                    (String::from("Credits"), Action::Push(Rc::new(TextBox::new(
                        Some(String::from("Credits")),
                        String::from("Game by Paul Maynard\nFlavor text contributed by:\n - Joyce Quach\ncurses_vector tileset by DragonDePlatino"),
                        50, 20, true
                    )))),
                    (String::from("Quit"), Action::Pop),
                ]
            )), help, seed));

            app.set_engine(Box::new(engine));
            console!(log, "game time started");
            app.run();
        }),
    );
}
