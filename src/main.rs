use std::env;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = env::args().nth(1).ok_or("No file path provided")?;
    let file = File::open(file_path).map_err(|_| "Could not open file")?;

    if payment_engine::app::runner::run(file, std::io::stdout()).is_err() {
        std::process::exit(1);
    } else {
        Ok(())
    }
}
