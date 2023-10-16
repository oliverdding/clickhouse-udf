#![feature(iter_array_chunks)]

use miette::{IntoDiagnostic, Result};
use std::{
    borrow::BorrowMut,
    io::{Read, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    match echo() {
        Ok(()) => {}
        Err(e) => {
            eprint!("failed execute echo: {}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

fn echo() -> Result<()> {
    let mut writer = std::io::stdout();

    let mut iter = std::io::stdin()
        .bytes()
        .map(|b| b.unwrap())
        .array_chunks::<8>();
    for chunk in iter.borrow_mut() {
        writer.write_all(&chunk).into_diagnostic()?;
    }
    {
        let iter = iter.into_remainder().unwrap();
        let remainder = iter.as_slice();
        if remainder.len() > 0 {
            eprintln!("expected size of bytes left: {:?}", remainder);
        }
    }

    writer.flush().into_diagnostic()?;

    Ok(())
}
