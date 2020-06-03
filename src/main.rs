use tcod::console::{ Root, FontLayout };

pub mod game;
pub mod screen;
use game::Game;
use screen::ScreenStack;
use screen::menu::MenuScreen;

const SCREEN_WIDTH: i32 = 100;
const SCREEN_HEIGHT: i32 = 45;

const LIMIT_FPS: i32 = 20;


fn main() {
    let root = Root::initializer()
        .font("curses_vector_16x24.png", FontLayout::AsciiInRow)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Example")
        .init();
    tcod::system::set_fps(LIMIT_FPS);

    let mut game = Game::new();

    let screens = ScreenStack::new(root);
    screens.play(&mut game, Box::new(MenuScreen::new(
        String::from(r#"
+-------------------------------------------------------------------------+
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
+-------------------------------------------------------------------------+
        "#),
        vec![
            String::from("Play!"),
            String::from("Help"),
            String::from("Credits"),
            String::from("Quit"),
        ]
    )));
}
