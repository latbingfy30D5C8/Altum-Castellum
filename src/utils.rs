use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufWriter, Write},
};

mod message;

pub fn u8_to_hex(byte: u8) -> String {
    let mut hex = String::new();
    let first_4_digit = byte & 0xF;
    let last_4_digit = byte >> 4;
    // i believe there is a way to remove the if block but i didn't figure it out yet
    if last_4_digit < 10 {
        String::push(&mut hex, (last_4_digit + 48) as char);
    } else {
        String::push(&mut hex, (last_4_digit + 87) as char);
    }
    if first_4_digit < 10 {
        String::push(&mut hex, (first_4_digit + 48) as char);
    } else {
        String::push(&mut hex, (first_4_digit + 87) as char);
    }
    hex
}

// How To Use: Opens a new "log" file if never opened. Takes the ownership of the writer and gives you back. You can drop it to flush the logs manually.
pub fn logit(
    msg: impl Debug,
    handle: Option<BufWriter<File>>,
) -> Result<BufWriter<File>, io::Error> {
    if handle.is_none() {
        let fileus = File::options().append(true).create(true).open("log")?;
        let mut buffer = BufWriter::with_capacity(2173, fileus);
        write!(&mut buffer, "{:?}", msg)?; // i regret write_all
        return Ok(buffer);
    }
    let mut buffer = handle.unwrap(); // mysterious space rays can kiss my ass
    write!(&mut buffer, "{:?}", msg)?;
    Ok(buffer)
}
