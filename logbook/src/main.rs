use anyhow::Result;
use logbook::append;
use logbook::read;
use std::env;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        if let Some(text) = read("logbook.txt")? {
            print!("{text}");
        } else {
            println!("Logbook is empty");
        }
    } else {
        append("logbook.txt", &args.join(" "))?;
    }
    Ok(())
}
