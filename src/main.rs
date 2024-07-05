use rand::Rng;

use hungry_runner::{self, Mission};

fn main() {
    let mut rng = rand::thread_rng();

    let mut mission = Mission::default();
    for _i in 0..50 {
        mission.add_dummy_task(rng.gen_range(0.0..3.0));
    }
    mission.run();
}
