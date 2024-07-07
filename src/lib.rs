use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{
    process::{Command, ExitStatus},
    time::Duration,
};

#[derive(Default)]
pub struct Mission {
    pub tasks: Vec<Box<dyn RunnableTask>>,
}

impl Mission {
    pub fn add_dummy_task(&mut self, sec: f64) {
        self.tasks.push(Box::new(DummyTask { duration: sec }))
    }
    pub fn run(&self) {
        let multi_progress = MultiProgress::new();
        let n: u64 = self.tasks.len().try_into().unwrap();
        let main_progress = ProgressBar::new(n);
        let main_style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.magenta} {pos:>7}/{len:7} {msg}",
        )
        .unwrap();

        let main_progress = multi_progress.insert(n.try_into().unwrap(), main_progress);
        main_progress.set_style(main_style);
        let numbered_tasks: Vec<(usize, &Box<dyn RunnableTask>)> =
            self.tasks.iter().enumerate().collect();

        let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
            .unwrap()
            //.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
            //.tick_chars("✶✸✹✺✹✷ ");
            //.tick_chars("┤┘┴└├┌┬┐ ");
            .tick_chars("▏▎▍▌▋▊▉▊▋▌▍▎ ");
        let _: Vec<()> = numbered_tasks
            .par_iter()
            .map(|(id, task)| {
                let mp_handle = multi_progress.insert_before(&main_progress, ProgressBar::new(1));
                mp_handle.set_style(spinner_style.clone());

                mp_handle.enable_steady_tick(Duration::from_millis(70));

                mp_handle.set_message(format!("Task {} running...", id));
                task.run();
                mp_handle.finish_and_clear();
                main_progress.inc(1);
                let _ = multi_progress.println(format!("Finished {id}"));
            })
            .collect();
        let _ = multi_progress.clear();
        main_progress.finish_and_clear();
    }
}

pub trait RunnableTask: Sync + Send {
    fn run(&self) -> std::process::Output;
}

pub struct DummyTask {
    duration: f64,
}

impl RunnableTask for DummyTask {
    fn run(&self) -> std::process::Output {
        let result = Command::new("sleep")
            .arg(format!("{}", self.duration))
            .output();
        return result.expect("failed to spawn child");
    }
}
