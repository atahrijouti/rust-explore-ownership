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
        println!("-----------------------------"); // Separator after each frame
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}

// Trait for common painter methods
trait Painter {
    fn draw_title(&mut self);
}

// MenuPainter handles drawing for the menu screen
struct MenuPainter {
    canvas: Canvas,
}

impl MenuPainter {
    fn new() -> Self {
        Self {
            canvas: Canvas::new(),
        }
    }

    fn draw_option(&mut self, option: u32) {
        self.canvas
            .add_string(&format!("Drawing menu option: {}", option));
    }
}

impl Painter for MenuPainter {
    fn draw_title(&mut self) {
        self.canvas.add_string("Drawing Menu Title");
    }
}

// GamePainter handles drawing for the game screen
struct GamePainter {
    canvas: Canvas,
}

impl GamePainter {
    fn new() -> Self {
        Self {
            canvas: Canvas::new(),
        }
    }

    fn draw_player(&mut self, player_x: u32) {
        self.canvas
            .add_string(&format!("Drawing player at position: {}", player_x));
    }
}

impl Painter for GamePainter {
    fn draw_title(&mut self) {
        self.canvas.add_string("Drawing Game Title");
    }
}

// Trait for screens, which must implement update and paint
trait Screen {
    fn update(&mut self) -> ScreenEvent;
    fn paint(&mut self);
    fn get_canvas(&mut self) -> &mut Canvas;
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
            painter: MenuPainter::new(),
        }
    }
}

impl Screen for MenuScreen {
    fn update(&mut self) -> ScreenEvent {
        self.selected_option += 1;
        if self.selected_option >= 3 {
            return ScreenEvent::SwitchToGame;
        }
        ScreenEvent::None
    }

    fn paint(&mut self) {
        self.painter.draw_title();
        self.painter.draw_option(self.selected_option);
    }

    fn get_canvas(&mut self) -> &mut Canvas {
        &mut self.painter.canvas
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
            painter: GamePainter::new(),
        }
    }
}

impl Screen for GameScreen {
    fn update(&mut self) -> ScreenEvent {
        self.player_x += 10;
        if self.player_x > 50 {
            return ScreenEvent::SwitchToMenu;
        }
        ScreenEvent::None
    }

    fn paint(&mut self) {
        self.painter.draw_title();
        self.painter.draw_player(self.player_x);
    }

    fn get_canvas(&mut self) -> &mut Canvas {
        &mut self.painter.canvas
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
    canvas: Canvas, // Use a simple Canvas here
}

impl Game {
    fn new() -> Self {
        let canvas = Canvas::new();
        let initial_screen = MenuScreen::new();
        Game {
            current_screen: Box::new(initial_screen),
            canvas,
        }
    }

    fn update(&mut self) {
        match self.current_screen.update() {
            ScreenEvent::SwitchToGame => {
                self.current_screen = Box::new(GameScreen::new());
            }
            ScreenEvent::SwitchToMenu => {
                self.current_screen = Box::new(MenuScreen::new());
            }
            ScreenEvent::None => {}
        }
    }

    fn paint(&mut self) {
        self.canvas.clear();
        self.current_screen.get_canvas().clear();

        self.current_screen.paint();

        let screen_canvas = self.current_screen.get_canvas();
        // Integrate the painter's canvas into the main canvas
        self.canvas
            .buffer
            .extend(screen_canvas.buffer.iter().cloned());
    }

    fn render_canvas(&self) {
        self.canvas.render();
    }
}

fn main() {
    let mut game = Game::new();

    // Game loop (simplified)
    for _ in 0..20 {
        game.update();
        game.paint();
        game.render_canvas();
        // The canvas is cleared at the beginning of paint
    }
}
