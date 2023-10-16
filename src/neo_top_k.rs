use bytes::BufMut;
use miette::{miette, IntoDiagnostic, Result};
use std::{
    borrow::BorrowMut,
    env,
    io::{Bytes, Read, Stdin, Write},
    process::ExitCode,
};
use topk::FilteredSpaceSaving;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let num = if args.len() < 2 {
        10_usize
    } else {
        match args[1].parse() {
            Ok(n) if n < 1 || n > 65536 => {
                eprintln!("k must be between 1 and 65536");
                return ExitCode::FAILURE;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to parse input k: {}", e);
                return ExitCode::FAILURE;
            }
        }
    };

    match top_k(num) {
        Ok(()) => {}
        Err(e) => {
            eprint!("failed execute neoTopK: {}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

fn top_k(k: usize) -> Result<()> {
    let mut iter = std::io::stdin().bytes();
    let mut writer = std::io::stdout();

    let mut topk = FilteredSpaceSaving::new(k);

    loop {
        let str_size = read_uleb128(iter.borrow_mut())?;
        if str_size == 0 {
            break;
        }
        let buf = iter
            .borrow_mut()
            .take(str_size as usize)
            .map(|b| b.unwrap())
            .collect();

        topk.insert(String::from_utf8(buf).into_diagnostic()?, 1);
    }

    let element_array = topk.into_sorted_vec();
    write_uleb128(&mut writer, element_array.len() as u64)?;
    for element in element_array {
        // write string
        write_uleb128(&mut writer, element.0.len() as u64)?;
        writer.write_all(element.0.as_bytes()).into_diagnostic()?;
        // write occurrence
        let mut buf = vec![];
        buf.put_u64_le(element.1.estimated_count());
        writer.write_all(&buf).into_diagnostic()?;
    }

    writer.flush().into_diagnostic()?;

    Ok(())
}

fn write_uleb128(target: &mut impl Write, x: u64) -> Result<usize> {
    let mut i = 0;
    let mut x = x;
    while x >= 0x80 {
        target.write_all(&[x as u8 | 0x80]).into_diagnostic()?;
        x >>= 7;
        i += 1;
    }
    target.write_all(&[x as u8]).into_diagnostic()?;
    Ok(i + 1)
}

fn read_uleb128(source: &mut Bytes<Stdin>) -> Result<u64> {
    let mut result = 0_u64;
    for i in 0..10 {
        match source.next() {
            Some(b) => match b {
                Ok(b) => {
                    if (b & 0x80) == 0 {
                        if i == 9 && b > 1 {
                            return Err(miette!("overflow when decoding uleb128"));
                        }
                        return Ok(result | (u64::from(b) << (7 * i)));
                    }
                    result |= u64::from(b & 0x7F) << (7 * i);
                }
                Err(err) => {
                    return Err(miette!(
                        "failed to read byte when decoding uleb128: {}",
                        err
                    ))
                }
            },
            None => {
                return Err(miette!("unexpected EOF whening decoding uleb128"));
            }
        }
    }
    Err(miette!("overflow when decoding uleb128"))
}
