use std::path;

use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;
use ggez::mint;
use ggez::{graphics, Context, ContextBuilder, GameResult};

mod log_buf;
mod utils;

use log_buf::LogBuf;

fn main() -> GameResult {
    let resources = path::PathBuf::from("./resources");

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .add_resource_path(resources)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx, "Key Logger")?;

    // Run!
    event::run(&mut ctx, &mut event_loop, &mut my_game)
}

struct MyGame {
    // Your state here...
    title: String,
    text: graphics::Text,
    log_buf: LogBuf,
}

impl MyGame {
    pub fn new(ctx: &mut Context, title: &str) -> GameResult<MyGame> {
        // Load/create resources such as images here.

        // The ttf file will be in your resources directory. We mounted the directory so that we
        // can omit the path here.

        let font = graphics::Font::new(ctx, "/ArchitectsDaughter.ttf")?;
        let text = graphics::Text::new(("Hello world!", font, 96.0));

        let my_game = MyGame {
            title: title.to_string(),
            text,
            log_buf: LogBuf::new(10),
        };

        Ok(my_game)
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn text_input_event(&mut self, ctx: &mut Context, character: char) {
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Back) {
            // Delete the last character
            self.log_buf.delete();
        } else {
            // Add Char to key logging buffer
            self.log_buf.add(character);
        }

        // Update the text field
        self.text.fragments_mut()[0].text = self.log_buf.get_text();
        self.text.fragments_mut()[0].color = Some(utils::random_color());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // Draw code here...

        // Set title window
        ggez::graphics::set_window_title(ctx, self.title.as_str());

        // The center point of the screen for the text
        let text_x: f32 = (ggez::graphics::size(ctx).0 / 2.0) - (self.text.width(ctx) as f32 / 2.0);
        let text_y: f32 =
            (ggez::graphics::size(ctx).1 / 2.0) - (self.text.height(ctx) as f32 / 2.0);

        // Drawables are drawn from their top-left corner
        let dest_point = mint::Point2 {
            x: text_x,
            y: text_y,
        };
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)
    }
}
