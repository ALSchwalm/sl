use cucumber_rust::{async_trait, Context, Cucumber, World};
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::time::Duration;

mod steps;

#[derive(Debug)]
pub struct SlError(String);

impl Display for SlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SlError {}

#[derive(Default)]
pub struct SlWorld {
    sl_bin_path: PathBuf,
    commandline: Option<String>,
    retcode: Option<i32>,
    stdout: Option<String>,
    stderr: Option<String>,
    duration: Option<Duration>,
}

#[async_trait(?Send)]
impl World for SlWorld {
    type Error = SlError;

    async fn new() -> Result<Self, SlError> {
        let bin_path = std::env::var("SL_BIN_PATH")
            .map_err(|_| SlError("Unable to read SL_BIN_PATH".to_string()))?;

        Ok(Self {
            sl_bin_path: bin_path.into(),
            ..std::default::Default::default()
        })
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<SlWorld>::new()
        // Specifies where our feature files exist
        .features(&["./tests/features"])
        // Adds the implementation of our steps to the runner
        .steps(steps::steps())
        // Add some global context for all the tests, like databases.
        .context(Context::new())
        // Parses the command line arguments if passed
        .cli()
        // Runs the Cucumber tests and then exists
        .run_and_exit()
        .await
}
