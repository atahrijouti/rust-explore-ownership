use std::cell::RefCell;
use std::rc::Rc;

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
    fn draw_title(&self);
}

// MenuPainter handles drawing for the menu screen
struct MenuPainter {
    canvas: Rc<RefCell<Canvas>>,
}

impl MenuPainter {
    fn new(canvas: Rc<RefCell<Canvas>>) -> Self {
        Self { canvas }
    }

    fn draw_option(&self, option: u32) {
        self.canvas
            .borrow_mut()
            .add_string(&format!("Drawing menu option: {}", option));
    }
}

impl Painter for MenuPainter {
    fn draw_title(&self) {
        self.canvas.borrow_mut().add_string("Drawing Menu Title");
    }
}

// GamePainter handles drawing for the game screen
struct GamePainter {
    canvas: Rc<RefCell<Canvas>>,
}

impl GamePainter {
    fn new(canvas: Rc<RefCell<Canvas>>) -> Self {
        Self { canvas }
    }

    fn draw_player(&self, player_x: u32) {
        self.canvas
            .borrow_mut()
            .add_string(&format!("Drawing player at position: {}", player_x));
    }
}

impl Painter for GamePainter {
    fn draw_title(&self) {
        self.canvas.borrow_mut().add_string("Drawing Game Title");
    }
}

// Trait for screens, which must implement update and paint
trait Screen {
    fn update(&mut self) -> ScreenEvent;
    fn paint(&self);
}

// MenuScreen uses MenuPainter and raises events based on its logic
struct MenuScreen {
    selected_option: u32,
    painter: MenuPainter,
}

impl MenuScreen {
    fn new(canvas: Rc<RefCell<Canvas>>) -> Self {
        MenuScreen {
            selected_option: 0,
            painter: MenuPainter::new(canvas),
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

// GameScreen uses GamePainter and raises events based on its logic
struct GameScreen {
    player_x: u32,
    painter: GamePainter,
}

impl GameScreen {
    fn new(canvas: Rc<RefCell<Canvas>>) -> Self {
        GameScreen {
            player_x: 0,
            painter: GamePainter::new(canvas),
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

enum ScreenEvent {
    None,
    SwitchToGame,
    SwitchToMenu,
}

// Game manager that handles screen switching based on events
struct Game {
    current_screen: Box<dyn Screen>,
    canvas: Rc<RefCell<Canvas>>,
}

impl Game {
    fn new() -> Self {
        let canvas = Rc::new(RefCell::new(Canvas::new()));
        let initial_screen = MenuScreen::new(Rc::clone(&canvas));
        Game {
            current_screen: Box::new(initial_screen),
            canvas,
        }
    }

    fn update(&mut self) {
        match self.current_screen.update() {
            ScreenEvent::SwitchToGame => {
                self.current_screen = Box::new(GameScreen::new(Rc::clone(&self.canvas)));
            }
            ScreenEvent::SwitchToMenu => {
                self.current_screen = Box::new(MenuScreen::new(Rc::clone(&self.canvas)));
            }
            ScreenEvent::None => {}
        }
    }

    fn paint(&self) {
        self.current_screen.paint();
    }

    fn render_canvas(&self) {
        self.canvas.borrow().render();
    }

    fn clear_canvas(&self) {
        self.canvas.borrow_mut().clear();
    }
}

fn main() {
    let mut game = Game::new();

    // Game loop (simplified)
    for _ in 0..10 {
        game.update();
        game.paint();
        game.render_canvas();
        game.clear_canvas();
    }
}
