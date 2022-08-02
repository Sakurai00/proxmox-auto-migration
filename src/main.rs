use anyhow::Result;
use systemstat::{saturating_sub_bytes, Platform, System};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let sys = System::new();

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x),
    }

    migrate().await;

    Ok(())
}

#[allow(dead_code)]
async fn migrate() {
    let _migrate = Command::new("qm")
        .arg("migrate")
        .arg("104") // VM ID
        .arg("RPI02-pve") // target
        .arg("--online") // live migration
        .spawn()
        .expect("migrate command failed to start")
        .wait()
        .await
        .expect("migrate command failed to run");
}

#[allow(dead_code)]
fn get_mem_ratio() -> Result<f64> {
    let sys = System::new();
    match sys.memory() {
        Ok(memory) => {
            let used = saturating_sub_bytes(memory.total, memory.free).as_u64();
            let total = memory.total.as_u64();
            //eprintln!("{}, {}", used, total);
            let ratio = used as f64 / total as f64;
            Ok(ratio)
        }
        Err(x) => return Err(anyhow::anyhow!("memory error: {}", x)),
    }
}
