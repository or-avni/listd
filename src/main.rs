use std::fs;

fn read_items() -> std::io::Result<()> {
    let items = fs::read_dir(".")?;

    for item in items {
        print!("{}  ", item?.path().display());
    }
    Ok(())
}

fn main() {
    let read = read_items();

    if let Err(err) = read {
        println!("An error has occured: {}", err);
    }
}
