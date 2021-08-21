use cursive::event::Callback;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, Palette};
use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
use cursive::views::Canvas;
use cursive::Cursive;

#[rustfmt::skip]
mod trains;

#[derive(Debug)]
enum Error {
    InvalidAnimation,
    InvalidFrame,
    EmptyAnimation,
}

type Result<T> = std::result::Result<T, Error>;

/// A struct representing a single frame of animation
struct Frame {
    text: Vec<String>,
}

impl Frame {
    /// Create a new frame of animation from the given lines of text
    ///
    /// This frame will be rendered with the lines left-aligned,
    /// directly on top of each other.
    fn new(text: Vec<String>) -> Result<Self> {
        Ok(Self { text })
    }

    /// Create a frame of animation from a single string
    fn from_str(text: &str) -> Result<Self> {
        Ok(Self {
            text: text
                .split("\n")
                .map(|line| line.to_string())
                .collect::<Vec<_>>(),
        })
    }

    fn width(&self) -> usize {
        self.text.iter().map(|line| line.len()).max().unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.text.len()
    }
}

struct Animation {
    frames: Vec<Frame>,
    current_frame_idx: usize,
    speed: usize,
    current_step: usize,
}

impl Animation {
    /// Create a new animation from a list of frames
    fn new(speed: usize, frames: Vec<Frame>) -> Result<Self> {
        if frames.len() == 0 {
            Err(Error::EmptyAnimation)
        } else {
            Ok(Self {
                frames,
                speed,
                current_frame_idx: 0,
                current_step: 0,
            })
        }
    }

    /// Create a new animation from a string
    ///
    /// Frames are expected to be delimited by two newlines
    fn from_str(speed: usize, text: &str) -> Result<Self> {
        let frames = text
            .split("\n\n\n")
            .map(|block| Frame::from_str(block))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            frames,
            speed,
            current_frame_idx: 0,
            current_step: 0,
        })
    }

    /// Advance the animation. This may update the current frame depending
    /// on the speed of the animation
    fn step(&mut self) {
        self.current_step += 1;
        if self.current_step == self.speed {
            self.current_frame_idx = (self.current_frame_idx + 1) % self.frames.len();
            self.current_step = 0;
        }
    }

    /// Get the current frame of the animation
    fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame_idx]
    }

    /// The maximum width of any frame in this animation
    fn width(&self) -> usize {
        self.frames
            .iter()
            .map(|frame| frame.width())
            .max()
            .expect("Unable to get frame width")
    }

    /// The maximum height of any frame in this animation
    fn height(&self) -> usize {
        self.frames
            .iter()
            .map(|frame| frame.height())
            .max()
            .expect("Unable to get frame height")
    }
}

struct TrainState {
    view_width: Option<usize>,
    view_height: Option<usize>,
    x: i32,
    y: i32,
    train_animation: Animation,
    smoke_animation: Animation,
    smoke_offset: usize,
}

impl TrainState {
    /// Create a new TrainState with the given animation
    fn new(train_animation: Animation, smoke_animation: Animation, smoke_offset: usize) -> Self {
        TrainState {
            view_width: None,
            view_height: None,
            x: 0,
            y: 0,
            train_animation,
            smoke_animation,
            smoke_offset,
        }
    }

    /// Determine whether the train has animated accross the screen
    fn complete(&self) -> bool {
        let max_width = std::cmp::max(self.train_animation.width(), self.smoke_animation.width());
        self.x < -((self.view_width.expect("Unknown view width") + max_width) as i32)
    }

    /// Print a string at the given coordinate
    ///
    /// This method differs from the Printer::print method because it allows
    /// negative coordinates more correctly. Printer::print will not print
    /// the entire string if the 'x' coordinate is negative; this method will
    /// instead print any visible portions.
    fn print_str_at(&self, text: impl AsRef<str>, printer: &cursive::Printer, coord: (i32, i32)) {
        if coord.0 > 0 {
            printer.print(coord, text.as_ref());
        } else {
            let hidden_chars = -coord.0 as usize;
            if hidden_chars >= text.as_ref().len() {
                return;
            }
            let line = &text.as_ref()[hidden_chars..];
            printer.print((0, coord.1), line);
        }
    }

    /// Render the current state to the given printer
    fn render(&self, printer: &cursive::Printer) {
        let x_offset = self.view_width.expect("Unknown view width") as i32 + self.x;

        // Center the train animation vertically
        let middle_row = self.view_height.expect("Unknown view height") as i32 / 2;
        let animation_height = self.train_animation.height() as i32;
        let y_offset = middle_row - animation_height / 2 + self.y;

        for (i, line) in self.smoke_animation.current_frame().text.iter().enumerate() {
            self.print_str_at(
                line,
                printer,
                (
                    x_offset + self.smoke_offset as i32,
                    y_offset + i as i32 - self.smoke_animation.height() as i32,
                ),
            );
        }

        for (i, line) in self.train_animation.current_frame().text.iter().enumerate() {
            self.print_str_at(line, printer, (x_offset, y_offset + i as i32));
        }
    }

    /// Advance the state of the train
    ///
    /// This updates both the animation state and the position of the
    /// animation on the canvas
    fn step(&mut self) {
        self.x -= 1;
        self.train_animation.step();
        self.smoke_animation.step();
    }
}

/// Setup the color theme and keybindings for cursive
fn init_cursive() -> cursive::CursiveRunnable {
    let mut siv = cursive::default();

    let mut theme = siv.current_theme().clone();
    let mut palette = Palette::default();
    palette[Background] = TerminalDefault;
    palette[Primary] = TerminalDefault;
    palette[View] = TerminalDefault;

    theme.shadow = false;
    theme.borders = BorderStyle::None;
    theme.palette = palette;

    siv.set_theme(theme);

    // We can quit by pressing `q` (for now)
    siv.add_global_callback('q', Cursive::quit);

    // Don't allowt the usual suspects to force an exit
    siv.clear_global_callbacks(cursive::event::Event::Exit);
    siv.clear_global_callbacks(cursive::event::Event::CtrlChar('c'));

    // This seems about right
    siv.set_fps(18);

    siv
}

fn main() {
    let mut siv = init_cursive();

    let state = TrainState::new(
        Animation::from_str(1, &trains::default_train_animation()).expect("Invalid animation"),
        Animation::from_str(5, &trains::default_smoke_animation()).expect("Invalid animation"),
        trains::DEFAULT_TRAIN_SMOKESTACK_OFFSET,
    );

    let canvas = Canvas::new(state)
        .with_draw(|state, printer| state.render(printer))
        .with_on_event(|state, event| match event {
            cursive::event::Event::Refresh => {
                state.step();
                let done = state.complete();
                cursive::event::EventResult::Consumed(Some(Callback::from_fn(
                    move |siv: &mut Cursive| {
                        if done {
                            siv.quit()
                        }
                    },
                )))
            }
            _ => cursive::event::EventResult::Ignored,
        })
        .with_required_size(|state, constraints| {
            // Now that there is a known viewport size, we can set it in the state
            state.view_width = Some(constraints.x);
            state.view_height = Some(constraints.y);
            constraints
        });

    siv.add_fullscreen_layer(canvas);

    // Run the event loop
    siv.run();
}
