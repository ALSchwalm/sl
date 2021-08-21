
pub const DEFAULT_TRAIN_SMOKESTACK_OFFSET: usize = 5;
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

pub fn default_train_animation() -> String {
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
    default_train_frames
        .collect::<Vec<_>>()
        .join("\n\n\n")
}

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

pub fn default_smoke_animation() -> String {
    DEFAULT_SMOKE.join("\n\n\n")
}
