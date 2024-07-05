use indicatif::{MultiProgress, ProgressBar};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{process::Command, time::Duration};

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
        let main_progress = ProgressBar::new(self.tasks.len().try_into().unwrap());
        let main_progress = multi_progress.add(main_progress);
        let numbered_tasks: Vec<(usize, &Box<dyn RunnableTask>)> =
            self.tasks.iter().enumerate().collect();

        let _: Vec<()> = numbered_tasks
            .par_iter()
            .map(|(id, task)| {
                let pb = ProgressBar::new_spinner();
                let mp_handle = multi_progress.add(pb);
                mp_handle.enable_steady_tick(Duration::from_millis(50));

                mp_handle.set_message(format!("Task {} running...", id));
                task.run();
                mp_handle.finish_and_clear();
                main_progress.inc(1);
            })
            .collect();
        let _ = multi_progress.clear();
        main_progress.finish_and_clear();
    }
}

pub trait RunnableTask: Sync + Send {
    fn run(&self);
}

pub struct DummyTask {
    duration: f64,
}

impl RunnableTask for DummyTask {
    fn run(&self) {
        let result = Command::new("sleep")
            .arg(format!("{}", self.duration))
            .spawn();
        result.unwrap().wait().unwrap();
    }
}
