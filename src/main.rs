use std::process::exit;

use clap::Parser;
use rand::Rng;

use hungry_runner::{self, cli, BashTask, Mission, RunnableTask, StringTask};

fn main() {
    let cli = cli::Cli::parse();

    let tasks: Vec<Box<dyn RunnableTask>> = if cli.bash {
        cli.comands
            .into_iter()
            .map(|x| -> Box<dyn RunnableTask> { Box::new(BashTask::new(x)) })
            .collect()
    } else {
        cli.comands
            .into_iter()
            .map(|x| -> Box<dyn RunnableTask> { Box::new(StringTask::new(x)) })
            .collect()
    };

    let mut rng = rand::thread_rng();

    let mut mission = Mission::default();

    for task in tasks {
        mission.add_task(task);
    }
    for _i in 0..cli.demo {
        mission.add_dummy_task(rng.gen_range(0.0..3.0));
    }
    let passing = mission.run();
    if !passing {
        exit(1);
    }
}
