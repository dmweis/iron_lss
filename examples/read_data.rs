use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Args {
    #[structopt(about = "Serial port to use")]
    port: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::from_args();
    let mut driver = lss_driver::LSSDriver::new(&args.port)?;
    println!("Voltage is {} V", driver.query_voltage(5).await?);
    println!("Temperature is {} °C", driver.query_temperature(5).await?);
    println!("Current is {} A", driver.query_current(5).await?);
    println!("Position is {} degrees", driver.query_position(5).await?);
    println!(
        "Filter position count is {}",
        driver.query_filter_position_count(5).await?
    );
    Ok(())
}
