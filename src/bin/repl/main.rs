#![allow(dead_code)]

// use genzlang::parse::stream::TokenStream;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let stdin_handle = io::stdin().lock();
    let stdout_handle = io::stdout().lock();
    start(stdin_handle, stdout_handle)
}

const PROMPT: &str = "🙏 -> ";

fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    // loop {
    //     write!(writer, "{}", PROMPT)?;
    //     writer.flush()?;
    //
    //     let mut buffer = String::new();
    //     reader.read_line(&mut buffer)?;
    //
    //     match buffer.as_str() {
    //         "exit\n" => break,
    //         source => {
    //             TokenStream::new(&source).try_for_each(|t| writeln!(writer, "{:?}", t))?;
    //         }
    //     };
    // }
    Ok(())
}
