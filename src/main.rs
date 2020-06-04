use std::rc::Rc;

use tcod::console::{ Root, FontLayout };

pub mod game;
pub mod screen;
use game::Game;
use screen::{ ScreenStack, Action };
use screen::menu::MenuScreen;
use screen::textbox::TextBox;

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

    let mut game = Game::new(Rc::new(MenuScreen::new(String::from(r#"+-------------------------------------------------------------------------+
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
            (String::from("Play!"), Action::Keep),
            (String::from("Help"), Action::Help),
            (String::from("Credits"), Action::Push(Rc::new(TextBox::new(
                Some(String::from("Credits")),
                String::from("Game by Paul Maynard\nFlavor text contributed by Joyce Quach\ncurses_vector tileset by DragonDePlatino"),
                50, 20, true
            )))),
            (String::from("Quit"), Action::Pop),
        ]
    )), Rc::new(TextBox::new(
        Some(String::from("Help")),
        String::from(r#"? - Show help screen"#),
        30, 30, true
    )));

    let screens = ScreenStack::new(root);
    screens.play(&mut game);
}
