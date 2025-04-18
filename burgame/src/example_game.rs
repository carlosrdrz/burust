use burengine::game::Game;
use burengine::graphics::Graphics;
use burengine::renderer::Renderer;
use burengine::{self, types::Color};
use crate::ui::pane::Pane;
use crate::ui::{square::Square, label::Label, button::Button, 
                input_box::InputBox, selector::Selector, image::Image};
use crate::ui_manager::UIManager;

pub struct ExampleGame {
    ui_manager: UIManager,
    finished: bool,
    i: u8,
    counter: u32,
    selected_option: String,
}

impl ExampleGame {
    pub fn new() -> ExampleGame {
        Self {
            ui_manager: UIManager::new(Vec::new()),
            finished: false,
            i: 0,
            counter: 0,
            selected_option: "Option 1".to_string(),
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
        let mut right_pane = Pane::new(520, 0, 220, 400);

        // Label widget
        let label = Label::new(10, 10, 200, 30, "Hello, World!");
        right_pane.add_widget(Box::new(label));

        // Button widget
        let button = Button::new(10, 50, 200, 40, "Click Me!");
        right_pane.add_widget(Box::new(button));

        // Input box widget
        let input_box = InputBox::new(10, 100, 200, 30, "Type here...");
        right_pane.add_widget(Box::new(input_box));

        // Selector widget
        let options = vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()];
        let selector = Selector::new(10, 140, 200, 30, options);
        right_pane.add_widget(Box::new(selector));

        // Image widget (assuming we have a test image)
        let image = Image::new(10, 180, 200, 200, "assets/test.png");
        right_pane.add_widget(Box::new(image));

        self.ui_manager.add_pane(right_pane);
    }

    fn main_loop(&mut self) {
        // Animate the square color
        self.i = (self.i + 1) % 255;
        let square_widget: &mut Square = self.ui_manager.get_pane_widget_as_mut(0, 0);
        square_widget.set_color(Color::RGB(self.i, 64, 255 - self.i));

        // Update label text
        let label: &mut Label = self.ui_manager.get_pane_widget_as_mut(1, 0);
        label.set_text(&format!("Counter: {}", self.counter));
        self.counter += 1;

        // Update button state based on counter
        let button: &mut Button = self.ui_manager.get_pane_widget_as_mut(1, 1);
        button.set_pressed(self.counter % 100 < 50);

        // Get selected option from selector
        let selector: &mut Selector = self.ui_manager.get_pane_widget_as_mut(1, 3);
        if let Some(option) = selector.get_selected_option() {
            self.selected_option = option.to_string();
        }
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
