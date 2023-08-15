use miette::{IntoDiagnostic, Result};
use polars::prelude::*;
use polars_io::ipc::IpcStreamReader;
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let num = if args.len() < 2 {
        10_usize
    } else {
        match args[1].parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to parse input k: {}", e);
                return ExitCode::FAILURE;
            }
        }
    };
    if num < 1 || num > 65536 {
        eprintln!("k must be between 1 and 65536");
        return ExitCode::FAILURE;
    }

    match top_k(num) {
        Ok(()) => {}
        Err(e) => {
            eprint!("failed execute top_k: {}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

fn top_k(k: usize) -> Result<()> {
    let df = IpcStreamReader::new(std::io::stdin())
        .finish()
        .into_diagnostic()?;

    if df.get_columns().len() == 0 {
        return Err(miette::miette!("no columns found"));
    }

    let mut df = df
        .top_k(k, false, vec![df.get_column_names()[0]])
        .into_diagnostic()?;

    IpcStreamWriter::new(std::io::stdout())
        .finish(&mut df)
        .into_diagnostic()?;
    Ok(())
}
