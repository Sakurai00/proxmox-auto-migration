use anyhow::Result;
use systemstat::{saturating_sub_bytes, Platform, System};
use tokio::process::Command;
use which::which;

#[tokio::main]
async fn main() -> Result<()> {
    match which("qm") {
        Ok(_) => println!("ProxmoxVE Installed"),
        Err(_) => println!("qm not found"),
    }
    println!("Hostname: {}", hostname::get()?.to_string_lossy());

    // test code
    let sys = System::new();
    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x),
    }
    migrate().await;

    //TODO 状態チェックし続ける．閾値を複数回連続で超えたらmigrate
    /* loop {
        let count = 0;

        // 1秒sleep

        if temp > 80 {
            count += 1;
        }

        if count >= 10 {
            migrate();
            std::process::exit(0)
        }
    } */

    Ok(())
}

#[allow(dead_code)]
async fn migrate() {
    //TODO 引数でVMIDとtargetを指定できるようにする．
    //TODO Resultを返す．
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
