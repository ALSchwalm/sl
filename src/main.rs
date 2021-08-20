use cursive::event::Callback;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, Palette};
use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
use cursive::views::Canvas;
use cursive::views::LayerPosition;
use cursive::Cursive;

#[derive(Debug)]
enum Error {
    EmptyAnimation,
    EmptyFrame,
}

type Result<T> = std::result::Result<T, Error>;

struct Frame {
    text: Vec<String>,
}

impl Frame {
    fn new(text: Vec<String>) -> Result<Self> {
        if text.len() == 0 {
            Err(Error::EmptyFrame)
        } else {
            Ok(Self { text })
        }
    }

    fn width(&self) -> usize {
        self.text
            .iter()
            .map(|line| line.len())
            .max()
            .expect("Unable to get largest line of frame")
    }
}

struct Animation {
    frames: Vec<Frame>,
    current_frame_idx: usize,
    speed: usize,
    current_step: usize,
}

impl Animation {
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

    fn step(&mut self) {
        self.current_step += 1;
        if self.current_step == self.speed {
            self.current_frame_idx = (self.current_frame_idx + 1) % self.frames.len();
            self.current_step = 0;
        }
    }

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
}

struct TrainState {
    view_width: Option<usize>,
    view_height: Option<usize>,
    x: i32,
    y: i32,
    animation: Animation,
}

impl TrainState {
    fn new(animation: Animation) -> Self {
        TrainState {
            view_width: None,
            view_height: None,
            x: 0,
            y: 0,
            animation: animation,
        }
    }

    fn complete(&self) -> bool {
        self.x < -((self.view_width.expect("Unknown view width") + self.animation.width()) as i32)
    }
}

fn main() {
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

    let state = TrainState::new(
        Animation::new(
            5,
            vec![
                Frame::new(vec![
                    "fooooooooooooooooooooooooooo".into(),
                    "baaaaaaaaaaaaaaaaaaaaaaaaaar".into(),
                ])
                .expect("Invalid frame"),
                Frame::new(vec![
                    "baaaaaaaaaaaaaaaaaaaaaaaaaar".into(),
                    "fooooooooooooooooooooooooooo".into(),
                ])
                .expect("Invalid frame"),
            ],
        )
        .expect("Invalid animation"),
    );

    let canvas = Canvas::new(state)
        .with_draw(|state, printer| {
            let x_offset = state.view_width.expect("Unknown view width") as i32 + state.x;
            let y_offset = state.view_height.expect("Unknown view height") as i32 / 2 + state.y;

            for (i, line) in state.animation.current_frame().text.iter().enumerate() {
                // Get the subsection of the line that should be renderd based
                // on the view
                // TODO: this should be cleaned up
                let (line, x_offset) = if x_offset < 0 {
                    let x_offset = (-x_offset) as usize;
                    if x_offset > line.len() {
                        ("".into(), 0)
                    } else {
                        ((&line[x_offset..]).to_string(), 0)
                    }
                } else {
                    (line.clone(), x_offset)
                };

                // NOTE: we don't need to handle negative y_offset+i here,
                // because cursive already handles that as expected (the
                // line that is 'off screen' will not be rendered)

                printer.print((x_offset, y_offset + i as i32), &line);
            }
        })
        .with_on_event(|state, event| match event {
            cursive::event::Event::Refresh => {
                state.x -= 1;
                state.animation.step();
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

    // We can quit by pressing `q` (for now)
    siv.add_global_callback('q', Cursive::quit);

    siv.add_fullscreen_layer(canvas);

    // This seems about right
    siv.set_fps(18);

    // Run the event loop
    siv.run();
}
