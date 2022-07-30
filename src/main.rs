use systemstat::{Platform, System, saturating_sub_bytes};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let sys = System::new();

    // let memory = sys.memory().expect("read error");
    // println!("CPU: {:#?}", memory);

    println!("mem ratio: {:#?}", get_mem_ratio()?);

    let sys = System::new();

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }

    Ok(())
}


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
        Err(x) => return Err(anyhow::anyhow!("memory error: {}", x))
    }
}