use anyhow::Result;
use anyhow::bail;
use std::env;

use weather::get_weather;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    let location = args.join(" ");

    if location.is_empty() {
        bail!("Usage: weather <LOCATION>")
    }

    let api_key = env::var("WEATHERSTACK_API_KEY")?;
    let weather = get_weather(&location, &api_key)?;

    println!("{weather}");
    Ok(())
}
