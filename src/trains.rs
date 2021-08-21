
const DEFAULT_TRAIN_TOP: &'static str =
"      ====        ________                ___________
  _D _|  |_______/        \\__I_I_____===__|_________|
   |(_)---  |   H\\________/ |   |        =|___ ___|
   /     |  |   H  |  |     |   |         ||_| |_||
  |      |  |   H  |__--------------------| [___] |
  | ________|___H__/__|_____/[][]~\\_______|       |
  |/ |   |-----------I_____I [][] []  D   |=======|__
";

const DEFAULT_TRAIN_WHEELS: [&'static str; 6] = [
"__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__
 |/-=|___|=    ||    ||    ||    |_____/~\\___/
  \\_/      \\O=====O=====O=====O_/      \\_/",

"__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__
 |/-=|___|=    ||    ||    ||    |_____/~\\___/
  \\_/      \\_O=====O=====O=====O/      \\_/",

"__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__
 |/-=|___|=   O=====O=====O=====O|_____/~\\___/
  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/",

"__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__
 |/-=|___|=    ||    ||    ||    |_____/~\\___/
  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/",

"__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__
 |/-=|___|=    ||    ||    ||    |_____/~\\___/
  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/",

"__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__
 |/-=|___|=O=====O=====O=====O   |_____/~\\___/
  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/",
];

pub fn default_train_animation() -> String {
    DEFAULT_TRAIN_WHEELS
        .iter()
        .map(|wheels| DEFAULT_TRAIN_TOP.to_string() + &wheels.to_string())
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
