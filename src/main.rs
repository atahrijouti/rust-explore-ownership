struct Canvas {
    log: String,
}

impl Canvas {
    pub fn draw_line(&mut self) {
        self.add_to_log("Wrote a fancy line");
    }
    pub fn draw_rect(&mut self) {
        self.add_to_log("Wrote a fancy rectangle");
    }
    fn add_to_log(&mut self, entry: &str) {
        self.log.push_str(&format!(".\n{entry}"));
    }
}

trait Screen {
    fn paint(&mut self, canvas: &mut Canvas);
}

struct MenuScreen;

impl Screen for MenuScreen {
    fn paint(&mut self, canvas: &mut Canvas) {
        canvas.draw_rect();
    }
}

struct GameScreen;

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
    screen: ScreenType,
    canvas: &'m mut Canvas,
}

enum ScreenType {
    Menu(MenuScreen),
    Game(GameScreen),
}

impl<'m> Manager<'m> {
    fn new(canvas: &'m mut Canvas) -> Self {
        Self {
            screen: ScreenType::Menu(MenuScreen),
            canvas,
        }
    }

    fn paint(&mut self) {
        match &mut self.screen {
            ScreenType::Menu(screen) => screen.paint(self.canvas),
            ScreenType::Game(screen) => screen.paint(self.canvas),
        }
    }

    fn swap_screen(&mut self, screen: ScreenName) {
        self.screen = match screen {
            ScreenName::Menu => ScreenType::Menu(MenuScreen),
            ScreenName::Game => ScreenType::Game(GameScreen),
        };
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

    // Expected messages:
    // Starting Point.
    // Wrote a fancy rectangle.
    // Wrote a fancy line
}
