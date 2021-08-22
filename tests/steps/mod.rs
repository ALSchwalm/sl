use cucumber_rust::Steps;
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use crate::SlWorld;

const HELPER_PATH: &'static str = "tests/steps/run_helper.sh";

fn run_sl_command(world: &SlWorld) -> Child {
    let mut cmd = Command::new(HELPER_PATH);
    cmd.arg(&world.sl_bin_path);

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    if let Some(ref cmdline) = world.commandline {
        cmd.args(cmdline.split(" "));
    }

    cmd.spawn().expect("Failed to spawn SL command")
}

fn run_sl_command_completion(mut world: SlWorld) -> SlWorld {
    let now = std::time::SystemTime::now();

    let mut child = run_sl_command(&world);
    let result = child.wait().expect("Failed to get SL result");

    world.duration = Some(now.elapsed().expect("Unable to get elapsed time"));
    world.retcode = result.code();

    let mut s = String::new();
    child
        .stdout
        .expect("Unable to get SL command stdout")
        .read_to_string(&mut s)
        .expect("Failed to read stdout");
    world.stdout = Some(s);

    let mut s = String::new();
    child
        .stderr
        .expect("Unable to get SL command stderr")
        .read_to_string(&mut s)
        .expect("Failed to read stderr");
    world.stderr = Some(s);

    world
}

pub fn steps() -> Steps<SlWorld> {
    let mut steps: Steps<SlWorld> = Steps::new();

    steps.given_regex(r#"an sl command line of "(.*)"$"#, |mut world, ctx| {
        world.commandline = Some(ctx.matches[1].clone());
        world
    });

    steps.given(r#"an sl command"#, |world, _ctx| world);

    steps.when("the command is run to completion", |world, _ctx| {
        run_sl_command_completion(world)
    });

    steps.then_regex(r#"the exit code is (\d+)$"#, |world, ctx| {
        let code = str::parse::<i32>(&ctx.matches[1]).expect("Invalid exit code");

        assert_eq!(world.retcode.expect("Unknown process retcode"), code);
        world
    });

    steps.then_regex(
        r#"the command took at least (\d+) seconds"#,
        |world, ctx| {
            let expected_duration = str::parse::<u64>(&ctx.matches[1]).expect("Invalid duration");
            let expected_duration = Duration::from_secs(expected_duration);

            assert!(world.duration.expect("Unknown process duration") > expected_duration);
            world
        },
    );

    steps
}
