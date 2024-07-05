use std::process::Command;

#[derive(Default)]
pub struct Mission {
    pub tasks: Vec<Box<dyn RunnableTask>>,
}

impl Mission {
    pub fn add_dummy_task(&mut self, sec: i32) {
        self.tasks.push(Box::new(DummyTask { duration: sec }))
    }
    pub fn run(&self) {
        self.tasks.iter().for_each(|task| task.run());
    }
}

pub trait RunnableTask {
    fn run(&self);
}

pub struct DummyTask {
    duration: i32,
}

impl RunnableTask for DummyTask {
    fn run(&self) {
        let result = Command::new("sleep")
            .arg(format!("{}", self.duration))
            .spawn();
        result.unwrap().wait().unwrap();
    }
}
