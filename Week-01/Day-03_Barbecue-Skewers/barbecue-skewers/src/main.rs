use std::fs::File;
use std::io::prelude::*;
use barbecue_skewers::{count_skewers};

fn main() -> std::io::Result<()> {

    let mut file = File::open("data/skewer.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let skewer_count = count_skewers(&buffer);

    println!("The number of vegetarian and non-vegetarian skewers are [{},{}], respectively", skewer_count[0], skewer_count[1]);

    Ok(())
}
