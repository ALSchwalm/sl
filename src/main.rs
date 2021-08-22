use clap::{App, Arg};
use cursive::event::Callback;
use cursive::theme::{BorderStyle, Color::*, Palette, PaletteColor::*};
use cursive::views::Canvas;
use cursive::Cursive;
use rand::{thread_rng, Rng};
use std::fs::read_dir;
use std::path::Path;

mod animation;
mod error;
mod trains;

use animation::Animation;
use error::{Error, Result};

const FLYING_RATE: i32 = 10;

struct SmokeState {
    animation: Animation,
    offset: usize,
}

struct TrainState {
    view_width: Option<usize>,
    view_height: Option<usize>,
    x: i32,
    y: i32,
    train_animation: Animation,
    smoke: Option<SmokeState>,
    flying: bool,
}

impl TrainState {
    /// Create a new TrainState with the given animation
    fn new(definition: &trains::TrainDefinition, flying: bool) -> Result<Self> {
        let train_animation = Animation::new(definition.train_animation_speed, &definition.train)?;

        let smoke = definition
            .smoke
            .as_ref()
            .map(|smoke| {
                Ok(SmokeState {
                    animation: Animation::new(
                        definition.smoke_animation_speed.unwrap_or(1),
                        &smoke,
                    )?,
                    offset: definition.smoke_offset.unwrap_or(0),
                })
            })
            .transpose()?;

        Ok(TrainState {
            view_width: None,
            view_height: None,
            x: 0,
            y: 0,
            flying,
            train_animation,
            smoke,
        })
    }

    /// Determine whether the train has animated accross the screen
    fn complete(&self) -> bool {
        let max_width = std::cmp::max(
            self.train_animation.width(),
            self.smoke
                .as_ref()
                .map(|smoke_state| smoke_state.animation.width())
                .unwrap_or(0),
        );
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

        if let Some(smoke_state) = &self.smoke {
            for (i, line) in smoke_state
                .animation
                .current_frame()
                .text
                .iter()
                .enumerate()
            {
                self.print_str_at(
                    line,
                    printer,
                    (
                        x_offset + smoke_state.offset as i32,
                        y_offset + i as i32 - smoke_state.animation.height() as i32,
                    ),
                );
            }
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

        // Fly up more slowly than to the left, because terminal
        // cells are rectangular, so less cells vertically than
        // horizontally
        if self.flying && self.x % FLYING_RATE == 0 {
            self.y -= 1;
        }

        if let Some(smoke_state) = &mut self.smoke {
            smoke_state.animation.step();
        }
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

    // Don't allowt the usual suspects to force an exit
    siv.clear_global_callbacks(cursive::event::Event::Exit);
    siv.clear_global_callbacks(cursive::event::Event::CtrlChar('c'));

    // This seems about right
    siv.set_fps(18);

    siv
}

/// Load a random train from a directory
fn load_random_file(
    rng: &mut impl rand::Rng,
    path: impl AsRef<Path>,
) -> Result<trains::TrainDefinition> {
    let candidates = read_dir(path)
        .map_err(Error::Io)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry.path().extension().and_then(|extension| {
                if extension.to_string_lossy() == "train" {
                    Some(entry.path())
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    if candidates.len() == 0 {
        return Err(Error::NoTrainsFound);
    }

    let choice = rng.gen_range(0..candidates.len());

    trains::TrainDefinition::from_file(&candidates[choice])
}

fn main() -> Result<()> {
    let cmdline = App::new("SL")
        .version("6.0")
        .about("")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .value_name("NUM")
                .help("Select a specific animation to run")
                .conflicts_with("dir")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("flying")
                .short("f")
                .long("flying")
                .help("The train flies up as well as left"),
        )
        .arg(
            Arg::with_name("accident")
                .short("a")
                .long("accident")
                .help("An accident happened")
                .conflicts_with("number")
                .conflicts_with("dir")
                .conflicts_with("logo")
                .conflicts_with("c51"),
        )
        .arg(
            Arg::with_name("c51")
                .short("c")
                .long("c51")
                .help("Show the C51 train")
                .conflicts_with("number")
                .conflicts_with("dir")
                .conflicts_with("logo")
                .conflicts_with("accident"),
        )
        .arg(
            Arg::with_name("logo")
                .short("l")
                .long("logo")
                .help("Show the LOGO train")
                .conflicts_with("number")
                .conflicts_with("dir")
                .conflicts_with("c51")
                .conflicts_with("accident"),
        )
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("directory")
                .value_name("DIR")
                .help("Load a random train from a directory")
                .takes_value(true),
        )
        .get_matches();

    let mut siv = init_cursive();

    let builtins = trains::builtin_trains();

    let mut rng = thread_rng();
    let random_train_idx: usize = rng.gen_range(0..builtins.len());

    let flying = cmdline.is_present("flying");

    let state = if cmdline.is_present("accident") {
        TrainState::new(&trains::accident_train(), flying)?
    } else if cmdline.is_present("c51") {
        TrainState::new(&trains::c51_train(), flying)?
    } else if cmdline.is_present("logo") {
        TrainState::new(&trains::logo_train(), flying)?
    } else if cmdline.is_present("dir") {
        let definition = load_random_file(&mut rng, cmdline.value_of("dir").unwrap())?;
        TrainState::new(&definition, flying)?
    } else {
        let train_idx = cmdline.value_of("number");
        let train_idx = train_idx
            .map(str::parse::<usize>)
            .transpose()
            .map_err(Error::IntParseError)?
            .unwrap_or(random_train_idx);

        if train_idx >= builtins.len() {
            return Err(Error::InvalidTrainIndex(train_idx));
        }

        TrainState::new(&builtins[train_idx], flying)?
    };

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
            // If we're flying, start at the bottom right
            // also, check the view_width state to determine if we've already
            // done this, to avoid reseting 'y' while running.
            if state.flying && state.view_width.is_none() {

                // Start at the bottom left, but shifted up by the height of the
                // train plus it's (optional) smoke
                state.y = constraints.y as i32
                    - (state.train_animation.height() as i32
                        + state
                            .smoke
                            .as_ref()
                            .map(|smoke| smoke.animation.height() as i32)
                            .unwrap_or(0));
            }

            // Now that there is a known viewport size, we can set it in the state
            state.view_width = Some(constraints.x);
            state.view_height = Some(constraints.y);

            constraints
        });

    siv.add_fullscreen_layer(canvas);

    // Run the event loop
    siv.run();

    Ok(())
}
