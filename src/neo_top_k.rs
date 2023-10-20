#![feature(buf_read_has_data_left)]
use bytes::BufMut;
use miette::{miette, IntoDiagnostic, Result};
use std::{
    borrow::BorrowMut,
    env,
    io::{BufRead, Read, Write},
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

    let reader = std::io::stdin().lock();
    let writer = std::io::stdout();
    match top_k(num, reader, writer) {
        Ok(()) => {}
        Err(e) => {
            eprint!("failed execute neoTopK: {}", e);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

fn top_k(k: usize, mut reader: impl BufRead, mut writer: impl Write) -> Result<()> {
    let mut topk = FilteredSpaceSaving::new(k);

    loop {
        match reader.has_data_left() {
            Ok(true) => {}
            Ok(false) => {
                break;
            }
            Err(e) => {
                eprintln!("failed to read: {}", e);
            }
        }
        let str_size = read_uleb128(&mut reader)?;
        if str_size == 0 {
            continue;
        }

        let mut buf = vec![0; str_size as usize];
        reader.read_exact(&mut buf).into_diagnostic()?;

        topk.insert(String::from_utf8(buf).into_diagnostic()?, 1);
    }

    let element_array = topk.into_sorted_vec();
    write_uleb128(&mut writer, std::cmp::min(k, element_array.len()) as u64)?; // user may pass less element than k, should return the smaller one

    let mut buf = vec![];
    for element in element_array {
        buf.clear();
        // write string
        write_uleb128(&mut buf, element.0.len() as u64)?;
        buf.put_slice(element.0.as_bytes());
        // write occurrence
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
        target.write(&[x as u8 | 0x80]).into_diagnostic()?;
        x >>= 7;
        i += 1;
    }
    target.write(&[x as u8]).into_diagnostic()?;
    Ok(i + 1)
}

fn read_uleb128(source: &mut impl Read) -> Result<u64> {
    let mut buf = [0_u8; 1];
    let mut result = 0_u64;
    for i in 0..10 {
        match source.read(buf.borrow_mut()) {
            Ok(1) => {
                if (buf[0] & 0x80) == 0 {
                    if i == 9 && buf[0] > 1 {
                        return Err(miette!("overflow when decoding uleb128"));
                    }
                    return Ok(result | (u64::from(buf[0]) << (7 * i)));
                }
                result |= u64::from(buf[0] & 0x7F) << (7 * i);
            }
            Ok(0) => {
                return Err(miette!("unexpected EOF whening decoding uleb128"));
            }
            Ok(_) => {
                unreachable!("impossible to reach here")
            }
            Err(err) => {
                return Err(miette!(
                    "failed to read byte when decoding uleb128: {}",
                    err
                ))
            }
        }
    }
    Err(miette!("overflow when decoding uleb128"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_uleb128_1() {
        let mut buf = Vec::<u8>::with_capacity(10);
        let mut f = Cursor::new(&mut buf);
        let len = write_uleb128(&mut f, 1).unwrap();
        assert_eq!(len, 1);
        assert_eq!(buf, [0x01]);
    }

    #[test]
    fn test_write_uleb128_2() {
        let mut buf = Vec::<u8>::with_capacity(10);
        let mut f = Cursor::new(&mut buf);
        let len = write_uleb128(&mut f, 2).unwrap();
        assert_eq!(len, 1);
        assert_eq!(buf, [0x02]);
    }

    #[test]
    fn test_write_uleb128_127() {
        let mut buf = Vec::<u8>::with_capacity(10);
        let mut f = Cursor::new(&mut buf);
        let len = write_uleb128(&mut f, 127).unwrap();
        assert_eq!(len, 1);
        assert_eq!(buf, [0x7f]);
    }

    #[test]
    fn test_write_uleb128_128() {
        let mut buf = Vec::<u8>::with_capacity(10);
        let mut f = Cursor::new(&mut buf);
        let len = write_uleb128(&mut f, 128).unwrap();
        assert_eq!(len, 2);
        assert_eq!(buf, [0x80, 0x01]);
    }

    #[test]
    fn test_write_uleb128_255() {
        let mut buf = Vec::<u8>::with_capacity(10);
        let mut f = Cursor::new(&mut buf);
        let len = write_uleb128(&mut f, 255).unwrap();
        assert_eq!(len, 2);
        assert_eq!(buf, [0xff, 0x01]);
    }

    #[test]
    fn test_write_uleb128_256() {
        let mut buf = Vec::<u8>::with_capacity(10);
        let mut f = Cursor::new(&mut buf);
        let len = write_uleb128(&mut f, 256).unwrap();
        assert_eq!(len, 2);
        assert_eq!(buf, [0x80, 0x02]);
    }

    #[test]
    fn test_write_uleb128_100500() {
        let mut buf = Vec::<u8>::with_capacity(10);
        let mut f = Cursor::new(&mut buf);
        let len = write_uleb128(&mut f, 100500).unwrap();
        assert_eq!(len, 3);
        assert_eq!(buf, [0x94, 0x91, 0x06]);
    }

    #[test]
    fn test_read_uleb128_1() {
        let mut buf = vec![0x01];
        let mut f = Cursor::new(&mut buf);
        let result = read_uleb128(&mut f).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_read_uleb128_2() {
        let mut buf = vec![0x02];
        let mut f = Cursor::new(&mut buf);
        let result = read_uleb128(&mut f).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_read_uleb128_127() {
        let mut buf = vec![0x7f];
        let mut f = Cursor::new(&mut buf);
        let result = read_uleb128(&mut f).unwrap();
        assert_eq!(result, 127);
    }

    #[test]
    fn test_read_uleb128_128() {
        let mut buf = vec![0x80, 0x01];
        let mut f = Cursor::new(&mut buf);
        let result = read_uleb128(&mut f).unwrap();
        assert_eq!(result, 128);
    }

    #[test]
    fn test_read_uleb128_255() {
        let mut buf = vec![0xff, 0x01];
        let mut f = Cursor::new(&mut buf);
        let result = read_uleb128(&mut f).unwrap();
        assert_eq!(result, 255);
    }

    #[test]
    fn test_read_uleb128_256() {
        let mut buf = vec![0x80, 0x02];
        let mut f = Cursor::new(&mut buf);
        let result = read_uleb128(&mut f).unwrap();
        assert_eq!(result, 256);
    }

    #[test]
    fn test_read_uleb128_100500() {
        let mut buf = vec![0x94, 0x91, 0x06];
        let mut f = Cursor::new(&mut buf);
        let result = read_uleb128(&mut f).unwrap();
        assert_eq!(result, 100500);
    }
}
