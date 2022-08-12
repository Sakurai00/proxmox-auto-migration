use anyhow::Result;
use duct::cmd;
use systemstat::{saturating_sub_bytes, Platform, System};
use tokio::process::Command;
use tokio::time::sleep;
use tokio::time::Duration;
use which::which;

#[tokio::main]
async fn main() -> Result<()> {
    match which("qm") {
        Ok(_) => println!("ProxmoxVE Installed"),
        Err(_) => println!("qm not found"),
    }
    let hostname = hostname::get()?;
    if let Some(n) = hostname.to_str() {
        println!("Hostname: {}", n);
    }

    let sys = System::new();
    let mut count = 0;
    loop {
        let cpu_temp = sys.cpu_temp()?;
        println!("CPU Temp: {}", cpu_temp);

        if cpu_temp > 35.0 {
            count += 1;
        } else {
            count = 0;
        }

        if count >= 10 {
            migrate(104, "RPI02-pve").await?;
            break;
        }

        sleep(Duration::from_millis(1000)).await;
    }

    Ok(())
}

async fn migrate(vmid: i64, target: &str) -> Result<()> {
    let migrate = cmd!("qm", "migrate", vmid.to_string(), target, "--online").run()?;

    if migrate.status.success() {
        println!("migrate success. VM:{} Target:{}", vmid, target);
        Ok(())
    } else {
        Err(anyhow::anyhow!("migrate error."))
    }
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
