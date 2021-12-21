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
    let mut driver = lss_driver::LSSDriver::new(&args.port).unwrap();
    driver
        .move_to_position(lss_driver::BROADCAST_ID, 0.0)
        .await?;
    driver.set_color(5, lss_driver::LedColor::Magenta).await?;
    Ok(())
}
