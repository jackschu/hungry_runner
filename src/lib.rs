use indicatif::{MultiProgress, ProgressBar};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{process::Command, time::Duration};

#[derive(Default)]
pub struct Mission {
    pub tasks: Vec<Box<dyn RunnableTask>>,
}

impl Mission {
    pub fn add_dummy_task(&mut self, sec: i32) {
        self.tasks.push(Box::new(DummyTask { duration: sec }))
    }
    pub fn run(&self) {
        let multi_progress = MultiProgress::new();

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
            })
            .collect();
        let _ = multi_progress.clear();
    }
}

pub trait RunnableTask: Sync + Send {
    fn run(&self);
}

pub struct DummyTask {
    duration: i32,
}

impl RunnableTask for DummyTask {
    fn run(&self) {
        let result = Command::new("sleep")
            .arg(format!("0.{}", self.duration))
            .spawn();
        result.unwrap().wait().unwrap();
    }
}
