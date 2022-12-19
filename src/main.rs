mod app;
mod crossterm;
mod ui;

use std::{error::Error, time::Duration};

pub fn main() -> Result<(), Box<dyn Error>> {
    const TICK_RATE : Duration = Duration::from_millis(200);
    crossterm::run(TICK_RATE)?;
    println!("hello world");
    Ok(())
}
