//! Default definitions of trains and train loading
//!
//! By default, sl provides a few builtin train types including:
//!
//! - A simple, standard train
//! - A C51 engine
//! - A LOGO train
//!
//! As well as an 'accident' train.

use crate::error::{Error, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// The defintion of a train
///
/// This type is effectively the boundary between user defintion and
/// the running, processed state of a train. A TrainDefinition
/// represents all the (potentially) user-defined properties that
/// describe what a train is and how it should move. Importantly,
/// this type does _not_ do validation. That will occur when the
/// definition is consumed to make a TrainState.
pub struct TrainDefinition {
    /// The main train animation body, as text suitable to be passed to Animation
    pub train: String,

    /// Speed of the train animation, as passed to Animation
    pub train_animation_speed: usize,

    /// Text of the smoke animation as would be passed to Animation
    pub smoke: Option<String>,

    /// Offset of the smoke from the train
    pub smoke_offset: Option<usize>,

    /// Speed of the smoke animation as passed to ANimation
    pub smoke_animation_speed: Option<usize>,
}

impl TrainDefinition {
    /// Load a train defintion from the given path
    ///
    /// Currently this only supports loading the train body animation.
    /// There will be no smoke, and speed is set to 1.
    pub fn from_file(path: impl AsRef<Path>) -> Result<TrainDefinition> {
        let mut file = File::open(path).map_err(Error::Io)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(Error::Io)?;
        Ok(TrainDefinition {
            train: contents,

            // NOTE: train speed is not configurable when reading from file,
            // also smoke is not supported
            train_animation_speed: 1,
            smoke: None,
            smoke_offset: None,
            smoke_animation_speed: None,
        })
    }
}

const DEFAULT_TRAIN_SMOKESTACK_OFFSET: usize = 5;
const DEFAULT_TRAIN_ANIMATION_SPEED: usize = 1;
const DEFAULT_TRAIN_SMOKE_SPEED: usize = 5;
const DEFAULT_TRAIN_TOP: &'static str = concat!(
    "      ====        ________                ___________ \n",
    "  _D _|  |_______/        \\__I_I_____===__|_________| \n",
    "   |(_)---  |   H\\________/ |   |        =|___ ___|   \n",
    "   /     |  |   H  |  |     |   |         ||_| |_||   \n",
    "  |      |  |   H  |__--------------------| [___] |   \n",
    "  | ________|___H__/__|_____/[][]~\\_______|       |   \n",
    "  |/ |   |-----------I_____I [][] []  D   |=======|___\n"
);

const DEFAULT_TRAIN_WHEELS: [&'static str; 6] = [
    concat!(
        "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__\n",
        " |/-=|___|=    ||    ||    ||    |_____/~\\___/        \n",
        "  \\_/      \\_O=====O=====O=====O/      \\_/            "
    ),
    concat!(
        "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__\n",
        " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        \n",
        "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "
    ),
    concat!(
        "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__\n",
        " |/-=|___|=    ||    ||    ||    |_____/~\\___/        \n",
        "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "
    ),
    concat!(
        "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__\n",
        " |/-=|___|=    ||    ||    ||    |_____/~\\___/        \n",
        "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "
    ),
    concat!(
        "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__\n",
        " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        \n",
        "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "
    ),
    concat!(
        "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__\n",
        " |/-=|___|=    ||    ||    ||    |_____/~\\___/        \n",
        "  \\_/      \\O=====O=====O=====O_/      \\_/            "
    ),
];

const DEFAULT_TRAIN_CAR: &'static str = "

    _________________
   _|                \\_____A
 =|                        |
 -|                        |
__|________________________|_
|__________________________|_
   |_D__D__D_|  |_D__D__D_|
    \\_/   \\_/    \\_/   \\_/";

const DEFAULT_SMOKE: [&'static str; 2] = [
    "                (  ) (@@) (  )  (@)  ()   @   O   @   O   @   O   @   O   @
             (@@@)
         (   )
     (@@@@)
  (    )

(@@@@)",
    "                (@@) (  ) (@@)  ( )  ()   O   @   O   @   O   @   O   @   O
             (   )
         (@@@)
     (    )
  (@@@@)

(    )",
];

fn build_composite_train(
    engine_top: &str,
    engine_wheels: &[&str],
    car: &str,
    smoke: &[&str],
    train_animation_speed: usize,
    smoke_animation_speed: usize,
    smoke_offset: usize,
) -> TrainDefinition {
    // Attach the wheels to the train
    let default_engine = engine_wheels
        .iter()
        .map(|wheels| engine_top.to_string() + wheels);

    // Attach the car to the engine
    let default_train_frames = default_engine.map(|frame| {
        frame
            .split("\n")
            .zip(car.split("\n"))
            .map(|(train_line, cart_line)| train_line.to_string() + cart_line)
            .collect::<Vec<_>>()
            .join("\n")
    });

    // Join the frames into a single string
    let train = default_train_frames.collect::<Vec<_>>().join("\n\n\n");

    let smoke = smoke.join("\n\n\n");

    TrainDefinition {
        train,
        train_animation_speed,
        smoke: Some(smoke),
        smoke_offset: Some(smoke_offset),
        smoke_animation_speed: Some(smoke_animation_speed),
    }
}

/// A definition for a simple, default train
pub fn default_train() -> TrainDefinition {
    build_composite_train(
        DEFAULT_TRAIN_TOP,
        &DEFAULT_TRAIN_WHEELS,
        DEFAULT_TRAIN_CAR,
        &DEFAULT_SMOKE,
        DEFAULT_TRAIN_ANIMATION_SPEED,
        DEFAULT_TRAIN_SMOKE_SPEED,
        DEFAULT_TRAIN_SMOKESTACK_OFFSET,
    )
}

const LOGO_TRAIN_SMOKESTACK_OFFSET: usize = 3;
const LOGO_TRAIN_ANIMATION_SPEED: usize = 1;
const LOGO_TRAIN_TOP: &'static str = concat!(
    "     ++      +------ \n",
    "     ||      |+-+ |  \n",
    "   /---------|| | |  \n",
    "  + ========  +-+ |  \n"
);

#[rustfmt::skip]
const LOGO_TRAIN_WHEELS: [&'static str; 6] = [
    concat!(" _|--/~\\------/~\\-+  \n",
            "//// O========O_/    "),

    concat!(" _|--/~\\------/~\\-+  \n",
            "//// \\O========O/    "),

    concat!(" _|--/~\\------/~\\-+  \n",
            "//// \\_O========O    "),

    concat!(" _|--/~O========O-+  \n",
            "//// \\_/      \\_/    "),

    concat!(" _|--/O========O\\-+  \n",
            "//// \\_/      \\_/    "),

    concat!(" _|--O========O~\\-+  \n",
            "//// \\_/      \\_/    "),
];

const LOGO_CARS: &'static str = concat!(
    "____                 ____________________ ____________________ \n",
    "|   \\@@@@@@@@@@@     |  ___ ___ ___ ___ | |  ___ ___ ___ ___ | \n",
    "|    \\@@@@@@@@@@@@@_ |  |_| |_| |_| |_| | |  |_| |_| |_| |_| | \n",
    "|                  | |__________________| |__________________| \n",
    "|__________________| |__________________| |__________________| \n",
    "   (O)       (O)        (O)       (O)        (O)       (O)     "
);

/// A definition for a LOGO train
pub fn logo_train() -> TrainDefinition {
    build_composite_train(
        LOGO_TRAIN_TOP,
        &LOGO_TRAIN_WHEELS,
        LOGO_CARS,
        &DEFAULT_SMOKE,
        LOGO_TRAIN_ANIMATION_SPEED,
        DEFAULT_TRAIN_SMOKE_SPEED,
        LOGO_TRAIN_SMOKESTACK_OFFSET,
    )
}

const C51_TRAIN_SMOKESTACK_OFFSET: usize = 7;
const C51_TRAIN_ANIMATION_SPEED: usize = 1;
const C51_TRAIN_TOP: &'static str = concat!(
    "        ___                                            \n",
    "       _|_|_  _     __       __             ___________\n",
    "    D__/   \\_(_)___|  |__H__|  |_____I_Ii_()|_________|\n",
    "     | `---'   |:: `--'  H  `--'         |  |___ ___|  \n",
    "    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  \n",
    "    ||        | ::       H  +=====+      |  |::  ...|  \n",
    "|    | _______|_::-----------------[][]-----|       |  \n"
);

const C51_TRAIN_WHEELS: [&'static str; 6] = [
    concat!(
        "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__\n",
        "------'|oOo|==[]=-     ||      ||      |  ||=======_|__\n",
        "/~\\____|___|/~\\_|   O=======O=======O  |__|+-/~\\_|     \n",
        "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       "
    ),
    concat!(
        "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__\n",
        "------'|oOo|===[]=-    ||      ||      |  ||=======_|__\n",
        "/~\\____|___|/~\\_|    O=======O=======O |__|+-/~\\_|     \n",
        "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       "
    ),
    concat!(
        "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__\n",
        "------'|oOo|===[]=- O=======O=======O  |  ||=======_|__\n",
        "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     \n",
        "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       "
    ),
    concat!(
        "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__\n",
        "------'|oOo|==[]=- O=======O=======O   |  ||=======_|__\n",
        "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     \n",
        "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       "
    ),
    concat!(
        "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__\n",
        "------'|oOo|=[]=- O=======O=======O    |  ||=======_|__\n",
        "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     \n",
        "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       "
    ),
    concat!(
        "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__\n",
        "------'|oOo|=[]=-      ||      ||      |  ||=======_|__\n",
        "/~\\____|___|/~\\_|  O=======O=======O   |__|+-/~\\_|     \n",
        "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       "
    ),
];

/// A definition for a C51 train engine
pub fn c51_train() -> TrainDefinition {
    // The C51 train is one taller than the default train, so the
    // car needs an extra newline
    let c51_train_car = "\n".to_string() + DEFAULT_TRAIN_CAR;

    build_composite_train(
        C51_TRAIN_TOP,
        &C51_TRAIN_WHEELS,
        &c51_train_car,
        &DEFAULT_SMOKE,
        C51_TRAIN_ANIMATION_SPEED,
        DEFAULT_TRAIN_SMOKE_SPEED,
        C51_TRAIN_SMOKESTACK_OFFSET,
    )
}

/// A definition for a train having an accident
pub fn accident_train() -> TrainDefinition {
    let mut def = default_train();

    def.train = include_str!("accident.train").to_string();

    def
}

/// A vector of all builtin trains
pub fn builtin_trains() -> Vec<TrainDefinition> {
    // Do not include 'accident' here as it isn't really a train type
    vec![default_train(), logo_train(), c51_train()]
}
