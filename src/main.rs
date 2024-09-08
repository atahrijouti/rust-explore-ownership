// Central canvas for collecting drawing instructions
struct Canvas {
    buffer: Vec<String>,
}

impl Canvas {
    fn new() -> Self {
        Canvas { buffer: Vec::new() }
    }

    fn add_string(&mut self, content: &str) {
        self.buffer.push(content.to_string());
    }

    fn render(&self) {
        for line in &self.buffer {
            println!("{}", line);
        }
        println!("-----------------------------"); // Add separator after each frame
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}

// Trait for common painter methods
trait Painter {
    fn draw_title(&self, canvas: &mut Canvas);
}

// MenuPainter handles drawing for the menu screen
struct MenuPainter;

impl MenuPainter {
    fn draw_option(&self, canvas: &mut Canvas, option: u32) {
        canvas.add_string(&format!("Drawing menu option: {}", option));
    }
}

impl Painter for MenuPainter {
    fn draw_title(&self, canvas: &mut Canvas) {
        canvas.add_string("Drawing Menu Title");
    }
}

// GamePainter handles drawing for the game screen
struct GamePainter;

impl GamePainter {
    fn draw_player(&self, canvas: &mut Canvas, player_x: u32) {
        canvas.add_string(&format!("Drawing player at position: {}", player_x));
    }
}

impl Painter for GamePainter {
    fn draw_title(&self, canvas: &mut Canvas) {
        canvas.add_string("Drawing Game Title");
    }
}

// Trait for screens, which must implement update and paint
trait Screen {
    fn update(&mut self) -> ScreenEvent;
    fn paint(&self, canvas: &mut Canvas);
}

// MenuScreen uses MenuPainter and raises events based on its logic
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

    fn paint(&self, canvas: &mut Canvas) {
        self.painter.draw_title(canvas);
        self.painter.draw_option(canvas, self.selected_option);
    }
}

// GameScreen uses GamePainter and raises events based on its logic
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

    fn paint(&self, canvas: &mut Canvas) {
        self.painter.draw_title(canvas);
        self.painter.draw_player(canvas, self.player_x);
    }
}

enum ScreenEvent {
    None,
    SwitchToGame,
    SwitchToMenu,
}

// Game manager that handles screen switching based on events
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

    fn paint(&self, canvas: &mut Canvas) {
        self.current_screen.paint(canvas);
    }
}

fn main() {
    let mut game = Game::new();
    let mut canvas = Canvas::new();

    // Game loop (simplified)
    for _ in 0..10 {
        game.update();
        game.paint(&mut canvas);
        canvas.render();
        canvas.clear();
    }
}
