struct Canvas {
    log: String,
}

impl Canvas {
    pub fn draw_line(&mut self) {
        self.add_to_log("Wrote a fancy line")
    }
    pub fn draw_rect(&mut self) {
        self.add_to_log("Wrote a fancy rectangle")
    }
    fn add_to_log(&mut self, entry: &str) {
        self.log.push_str(&format!("\n{entry}"));
    }
}

trait Screen {
    fn paint(&mut self, canvas: &mut Canvas);
}

struct MenuScreen {}

impl MenuScreen {
    fn new() -> MenuScreen {
        MenuScreen {}
    }
}

impl Screen for MenuScreen {
    fn paint(&mut self, canvas: &mut Canvas) {
        canvas.draw_rect();
    }
}

struct GameScreen {}

impl GameScreen {
    fn new() -> GameScreen {
        GameScreen {}
    }
}

impl Screen for GameScreen {
    fn paint(&mut self, canvas: &mut Canvas) {
        canvas.draw_line();
    }
}

pub enum ScreenName {
    Menu,
    Game,
}

struct Manager<'m> {
    screen: Box<dyn Screen + 'm>,
    canvas: &'m mut Canvas,
}

impl<'m> Manager<'m> {
    fn new(canvas: &'m mut Canvas) -> Manager<'m> {
        Manager {
            screen: Box::new(MenuScreen {}),
            canvas,
        }
    }

    fn paint(&mut self) {
        self.screen.paint(self.canvas)
    }

    fn swap_screen(&mut self, screen: ScreenName) {
        match screen {
            ScreenName::Menu => self.screen = Box::new(MenuScreen::new()),
            ScreenName::Game => self.screen = Box::new(GameScreen::new()),
        }
    }

    fn present(&self) -> &String {
        &self.canvas.log
    }
}

fn main() {
    let mut canvas = Canvas {
        log: String::from("Starting Point"),
    };

    let mut manager = Manager::new(&mut canvas);
    manager.paint();
    manager.swap_screen(ScreenName::Game);
    manager.paint();

    println!("{}", manager.present());

    // Expected messages

    // Starting Point.
    // Wrote a fancy rectangle.
    // Wrote a fancy line
}
