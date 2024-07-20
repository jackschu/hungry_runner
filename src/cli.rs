use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Space separated with format: <binary> <rest> eg: `"echo 'hello world;'" "sleep 3"`
    pub comands: Vec<String>,

    /// Interpret commands a space separate strings to pass to bash -c: eg `"sleep 1 && echo hi"`
    #[arg(short, long, action)]
    pub bash: bool,

    /// Number of dummy demo tasks to include
    #[arg(long, default_value_t = 0)]
    pub demo: u8,
}
