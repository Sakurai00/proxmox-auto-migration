use systemstat::{Platform, System};

fn main() {
    let sys = System::new();
    let memory = sys.memory().expect("read error");
    println!("CPU: {:#?}", memory);
}
