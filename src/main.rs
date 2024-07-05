use hungry_runner::{self, Mission};

fn main() {
    let mut mission = Mission::default();
    mission.add_dummy_task(1);
    mission.add_dummy_task(2);
    mission.add_dummy_task(3);
    mission.run();
    println!("Hello, world!");
}
