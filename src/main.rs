use cursive::views::Canvas;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, Palette};
use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
use cursive::Cursive;

struct Frame {
    text: Vec<String>
}

struct Animation {
    frames: Vec<Frame>,
    current_frame: usize
}

struct TrainState {
    x: usize,
    y: usize,
    animation: Animation
}

impl TrainState {
    fn new() -> Self {
        TrainState {
            x: 0,
            y: 0,
            animation: Animation {
                frames: vec![
                    Frame {
                        text: vec![
                            "foo".into(),
                            "bar".into()
                        ]
                    }
                ],
                current_frame: 0
            }
        }
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

    let state = TrainState::new();

    let canvas = Canvas::new(state)
        .with_draw(|state, printer| {
            let moved_printer = printer.offset(cursive::XY {
                                    x: state.x,
                                    y: state.y
                                });

            let current_frame = &state.animation.frames[state.animation.current_frame];
            for (i, line) in current_frame.text.iter().enumerate() {
                moved_printer.print((0, i), line);
            }

        })
        .with_on_event(|state, _event|{
            state.x += 1;

            // Do not consume the refresh event so we still trigger the global
            // refresh callback (which actually moves the canvas around)
            cursive::event::EventResult::Ignored
        })
        .with_required_size(|_state, _constraints| {
            // Really this should just be '2x the width of the terminal'
            (100, 100).into()
        });

    // We can quit by pressing `q` (for now)
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(canvas);

    // This seems about right
    siv.set_fps(18);

    // Run the event loop
    siv.run();
}
