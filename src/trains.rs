pub struct TrainDefinition {
    pub train: String,
    pub train_animation_speed: usize,
    pub smoke: Option<String>,
    pub smoke_offset: Option<usize>,
    pub smoke_animation_speed: Option<usize>
}

pub const DEFAULT_TRAIN_SMOKESTACK_OFFSET: usize = 5;
pub const DEFAULT_TRAIN_ANIMATION_SPEED: usize = 1;
pub const DEFAULT_TRAIN_SMOKE_SPEED: usize = 5;
const DEFAULT_TRAIN_TOP: &'static str =
    concat!("      ====        ________                ___________ \n",
            "  _D _|  |_______/        \\__I_I_____===__|_________| \n",
            "   |(_)---  |   H\\________/ |   |        =|___ ___|   \n",
            "   /     |  |   H  |  |     |   |         ||_| |_||   \n",
            "  |      |  |   H  |__--------------------| [___] |   \n",
            "  | ________|___H__/__|_____/[][]~\\_______|       |   \n",
            "  |/ |   |-----------I_____I [][] []  D   |=======|___\n");

const DEFAULT_TRAIN_WHEELS: [&'static str; 6] = [
    concat!("__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__\n",
            " |/-=|___|=    ||    ||    ||    |_____/~\\___/        \n",
            "  \\_/      \\O=====O=====O=====O_/      \\_/            "),

    concat!("__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__\n",
            " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        \n",
            "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "),

    concat!("__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__\n",
            " |/-=|___|=    ||    ||    ||    |_____/~\\___/        \n",
            "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "),

    concat!("__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__\n",
            " |/-=|___|=    ||    ||    ||    |_____/~\\___/        \n",
            "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "),

    concat!("__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__\n",
            " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        \n",
            "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "),

    concat!("__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__\n",
            " |/-=|___|=    ||    ||    ||    |_____/~\\___/        \n",
            "  \\_/      \\_O=====O=====O=====O/      \\_/            ")
];

const DEFAULT_TRAIN_CAR: &'static str =
"

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

pub fn default_train() -> TrainDefinition {
    // Attach the wheels to the train
    let default_engine = DEFAULT_TRAIN_WHEELS
        .iter()
        .map(|wheels| DEFAULT_TRAIN_TOP.to_string() + wheels);

    // Attach the car to the engine
    let default_train_frames = default_engine.map(|frame| {
            frame.split("\n")
                .zip(DEFAULT_TRAIN_CAR.split("\n"))
                .map(|(train_line, cart_line)|{
                    train_line.to_string() + cart_line
                }).collect::<Vec<_>>().join("\n")
        });

    // Join the frames into a single string
    let train = default_train_frames
        .collect::<Vec<_>>()
        .join("\n\n\n");

    let smoke = DEFAULT_SMOKE.join("\n\n\n");

    TrainDefinition {
        train,
        train_animation_speed: DEFAULT_TRAIN_ANIMATION_SPEED,
        smoke: Some(smoke),
        smoke_offset: Some(DEFAULT_TRAIN_SMOKESTACK_OFFSET),
        smoke_animation_speed: Some(DEFAULT_TRAIN_SMOKE_SPEED)
    }
}
