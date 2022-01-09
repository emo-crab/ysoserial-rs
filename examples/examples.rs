use std::fs::File;
use std::io;
use std::io::Write;
use ysoserial_rs::get_commons_beanutils1;

fn main() -> Result<(), io::Error> {
    let mut file = File::create("commons_beanutils1.ser")?;
    file.write_all(&get_commons_beanutils1("id"))?;
    Ok(())
}
