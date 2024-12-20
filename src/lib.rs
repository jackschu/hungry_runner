use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{process::Command, time::Duration};

#[derive(Default)]
pub struct Mission {
    pub tasks: Vec<Box<dyn RunnableTask>>,
}

impl Mission {
    pub fn add_task(&mut self, task: Box<dyn RunnableTask>) {
        self.tasks.push(task)
    }
    pub fn add_dummy_task(&mut self, sec: f64) {
        self.add_task(Box::new(DummyTask { duration: sec }))
    }
    pub fn run(&self) -> bool {
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
        let pass_fail: Vec<bool> = numbered_tasks
            .par_iter()
            .map(|(_id, task)| {
                let title = task.title();
                let mp_handle = multi_progress.insert_before(&main_progress, ProgressBar::new(1));
                mp_handle.set_style(spinner_style.clone());

                mp_handle.enable_steady_tick(Duration::from_millis(70));

                mp_handle.set_message(format!("Task: {} running...", title));
                let result = task.run();
                mp_handle.finish_and_clear();
                main_progress.inc(1);

                
                let out = result.is_ok();
                    
                let update_string = match result {

                    Ok(result) => {
                        if result.status.success() {
                            format!("{} Task: {title}", console::style("✓").green())
                        } else {
                            format!(
                                "{} Task: {title} {} (exit code: {})",
                                console::style("✗").red(),
                                console::style("Failed").red(),
                                result
                                    .status
                                    .code()
                                    .map(|x| x.to_string())
                                    .unwrap_or("None".to_string()),
                            )
                        }
                    }
                    Err(err) => format!(
                        "{} Task: {title} {} (error msg: {})",
                        console::style("✗").red(),
                        console::style("Failed").red(),
                        err,
                    ),
                };

                let _ = multi_progress.println(format!(" {update_string}"));
                out
            })
            .collect();
        let _ = multi_progress.clear();
        main_progress.finish_and_clear();
        pass_fail.into_iter().all(|x|x)
    }
}

pub trait RunnableTask: Sync + Send {
    fn run(&self) -> Result<std::process::Output, String>;
    fn title(&self) -> String;
}

pub struct BashTask {
    user_string: String,
}

impl BashTask {
    pub fn new(input: String) -> Self {
        Self { user_string: input }
    }
}

impl RunnableTask for BashTask {
    fn run(&self) -> Result<std::process::Output, String> {
        let result = Command::new("bash")
            .args(&["-c", self.user_string.as_str()])
            .output();
        return result.map_err(|err| err.to_string());
    }
    fn title(&self) -> String {
        self.user_string.clone()
    }
}

pub struct StringTask {
    user_string: String,
}

impl StringTask {
    pub fn new(input: String) -> Self {
        Self { user_string: input }
    }
}

impl RunnableTask for StringTask {
    fn run(&self) -> Result<std::process::Output, String> {
        let result = if let Some((binary, args)) = self.user_string.split_once(" ") {
            let arg_array: Vec<_> = args.split(' ').collect();
            Command::new(binary).args(arg_array).output()
        } else {
            Command::new(self.user_string.clone()).output()
        };
        return result.map_err(|err| err.to_string());
    }
    fn title(&self) -> String {
        self.user_string.clone()
    }
}

pub struct DummyTask {
    duration: f64,
}

impl RunnableTask for DummyTask {
    fn run(&self) -> Result<std::process::Output, String> {
        let result = Command::new("sleep")
            .arg(format!("{}", self.duration))
            .output();
        return result.map_err(|err| err.to_string());
    }
    fn title(&self) -> String {
        "Sleep command".to_string()
    }
}

pub mod cli;
