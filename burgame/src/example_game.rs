use burengine::game::Game;
use burengine::graphics::Graphics;
use burengine::renderer::Renderer;
use burengine::{self, types::Color};
use burengine::ui::{manager::Manager, pane::Pane, square::Square, label::Label, button::Button, 
                input_box::InputBox, selector::Selector, image::Image};

const UI_CONFIG_PATH: &str = "resources/ui_config.json";
pub struct ExampleGame {
    ui_manager: Manager,
    finished: bool,
    counter: u32,
}

impl ExampleGame {
    pub fn new() -> ExampleGame {
        Self {
            ui_manager: Manager::new(UI_CONFIG_PATH),
            finished: false,
            counter: 0,
        }
    }
}

impl Game for ExampleGame {
    fn init(&mut self) {
        // Main pane with square widget
        let color = Color::RGB(23, 23, 23);
        let widget = Square::new(20, 20, 460, 460, color);
        let mut main_pane = Pane::new(10, 10, 500, 500);
        main_pane.add_widget(Box::new(widget));
        self.ui_manager.add_pane(main_pane);

        // Right panel with all other widgets
        let mut right_pane = Pane::new(520, 10, 220, 400);

        // Label widget
        let label = Label::new(10, 10, 200, 30, "Hello, World!");
        right_pane.add_widget(Box::new(label));

        // Button widget
        let button = Button::new(10, 50, 200, 30, "Click Me!");
        right_pane.add_widget(Box::new(button));

        // Input box widget
        let input_box = InputBox::new(10, 100, 200, 20, "Type here...");
        right_pane.add_widget(Box::new(input_box));

        // Selector widget
        let options = vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()];
        let selector = Selector::new(10, 140, 200, 25, options);
        right_pane.add_widget(Box::new(selector));

        // Image widget (assuming we have a test image)
        let image = Image::new(10, 180, 140, 180, "resources/sprites/chars_avatar/1.png");
        right_pane.add_widget(Box::new(image));

        self.ui_manager.add_pane(right_pane);
    }

    fn main_loop(&mut self) {
        // Animate the square color
        let color_r = (self.counter % 255) as u8;
        let color_b = (255 - self.counter % 255) as u8;
        let square_widget: &mut Square = self.ui_manager.get_pane_widget_as_mut(0, 0);
        square_widget.set_color(Color::RGB(color_r, 64, color_b));

        // Update label text
        let label: &mut Label = self.ui_manager.get_pane_widget_as_mut(1, 0);
        label.set_text(&format!("Counter: {}", self.counter));
        self.counter += 1;

        // Update button state based on counter
        let button: &mut Button = self.ui_manager.get_pane_widget_as_mut(1, 1);
        button.set_pressed(self.counter % 100 < 50);
    }

    fn render(&self, graphics: &mut Graphics) {
        self.ui_manager.render(0, graphics);
    }

    fn end(&mut self) {
        self.finished = true;
    }

    fn is_done(&self) -> bool {
        self.finished
    }
}
