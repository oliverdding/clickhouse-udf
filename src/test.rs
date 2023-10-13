#![feature(buf_read_has_data_left)]

use miette::{IntoDiagnostic, Result};
use std::{
    io::{BufRead, Read, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    match test() {
        Ok(()) => {}
        Err(e) => {
            eprint!("failed execute test: {}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

fn test() -> Result<()> {
    let mut reader = std::io::stdin().lock();
    let mut writer = std::io::stdout().lock();

    let mut buf = vec![0_u8; 8];
    while reader.has_data_left().into_diagnostic()? {
        reader.read_exact(&mut buf).into_diagnostic()?;
        writer.write_all(&buf).into_diagnostic()?;
    }

    writer.flush().into_diagnostic()?;
    Ok(())
}
