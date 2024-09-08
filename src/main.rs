enum ScreenEvent {
    None,
    SwitchToGame,
    SwitchToMenu,
}

trait Painter {
    fn draw_title(&self);
}

struct MenuPainter;

impl MenuPainter {
    fn draw_option(&self, option: u32) {
        println!("Drawing menu option: {}", option);
    }
}

impl Painter for MenuPainter {
    fn draw_title(&self) {
        println!("Drawing Menu Title");
    }
}

struct GamePainter;

impl GamePainter {
    fn draw_player(&self, player_x: u32) {
        println!("Drawing player at position: {}", player_x);
    }
}

impl Painter for GamePainter {
    fn draw_title(&self) {
        println!("Drawing Game Title");
    }
}

trait Screen {
    fn update(&mut self) -> ScreenEvent;
    fn paint(&self);
}

struct MenuScreen {
    selected_option: u32,
    painter: MenuPainter,
}

impl MenuScreen {
    fn new() -> Self {
        MenuScreen {
            selected_option: 0,
            painter: MenuPainter,
        }
    }
}

impl Screen for MenuScreen {
    fn update(&mut self) -> ScreenEvent {
        self.selected_option += 1;
        if self.selected_option >= 4 {
            return ScreenEvent::SwitchToGame;
        }
        ScreenEvent::None
    }

    fn paint(&self) {
        self.painter.draw_title();
        self.painter.draw_option(self.selected_option);
    }
}

struct GameScreen {
    player_x: u32,
    painter: GamePainter,
}

impl GameScreen {
    fn new() -> Self {
        GameScreen {
            player_x: 0,
            painter: GamePainter,
        }
    }
}

impl Screen for GameScreen {
    fn update(&mut self) -> ScreenEvent {
        self.player_x += 5;
        if self.player_x > 100 {
            return ScreenEvent::SwitchToMenu;
        }
        ScreenEvent::None
    }

    fn paint(&self) {
        self.painter.draw_title();
        self.painter.draw_player(self.player_x);
    }
}

struct Game {
    current_screen: Box<dyn Screen>,
}

impl Game {
    fn new() -> Self {
        Game {
            current_screen: Box::new(MenuScreen::new()),
        }
    }

    fn update(&mut self) {
        match self.current_screen.update() {
            ScreenEvent::SwitchToGame => self.current_screen = Box::new(GameScreen::new()),
            ScreenEvent::SwitchToMenu => self.current_screen = Box::new(MenuScreen::new()),
            ScreenEvent::None => {}
        }
    }

    fn paint(&self) {
        self.current_screen.paint();
    }
}

fn main() {
    let mut game = Game::new();

    // Game loop (simplified)
    for _ in 0..6 {
        game.update();
        game.paint();
    }
}
