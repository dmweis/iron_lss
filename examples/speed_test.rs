use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Args {
    #[structopt(about = "Serial port to use")]
    port: String,
    #[structopt(
        about = "ID of the motor you want to move. Default BROADCAST",
        long = "id",
        default_value = "254"
    )]
    id: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::from_args();
    let mut driver = lss_driver::LSSDriver::new(&args.port).unwrap();
    driver.set_maximum_speed(args.id, 720.0).await?;
    driver.set_rotation_speed(args.id, 720.0).await?;
    driver
        .set_color(args.id, lss_driver::LedColor::Magenta)
        .await?;
    println!("Press enter to stop");
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    println!("{}", line);
    driver.set_rotation_speed(args.id, 0.0).await?;
    Ok(())
}
