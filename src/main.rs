use anyhow::Result;
use duct::cmd;
use log::*;
use simplelog::*;
use std::fs::File;
use systemstat::{saturating_sub_bytes, Platform, System};
use tokio::time::sleep;
use tokio::time::Duration;
use which::which;

#[tokio::main]
async fn main() -> Result<()> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("auto_migration.log").unwrap(),
        ),
    ])
    .unwrap();

    match which("qm") {
        Ok(_) => info!("ProxmoxVE Installed"),
        Err(_) => error!("qm not found"),
    }
    let hostname = hostname::get()?;
    if let Some(n) = hostname.to_str() {
        info!("Hostname: {}", n);
    }

    let sys = System::new();
    let mut count = 0;
    loop {
        let cpu_temp = sys.cpu_temp()?;
        info!("CPU Temp: {}", cpu_temp);

        if cpu_temp > 35.0 {
            count += 1;
            warn!("CPU Temp is Higher than 35.0 Count:{}", count);
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
    info!("Start migration. VM:{} Target:{}", vmid, target);
    let migrate = cmd!("qm", "migrate", vmid.to_string(), target, "--online").run()?;

    if migrate.status.success() {
        info!("Migration success. VM:{} Target:{}", vmid, target);
        Ok(())
    } else {
        error!("Migration error. VM:{} Target:{}", vmid, target);
        Err(anyhow::anyhow!("Migrate error."))
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
