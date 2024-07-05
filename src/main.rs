use hungry_runner::{self, Mission};

fn main() {
    let mut mission = Mission::default();
    for _i in 1..20 {
        mission.add_dummy_task(1);
        mission.add_dummy_task(2);
        mission.add_dummy_task(3);
    }
    mission.run();
}
