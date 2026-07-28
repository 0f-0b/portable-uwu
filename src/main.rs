use std::io::{self, Read, Write as _};

use portable_uwu::uwuify_into;

fn read_as_much_as_possible(mut r: impl Read, buf: &mut [u8]) -> io::Result<usize> {
    let mut pos = 0;
    while pos < buf.len() {
        match r.read(&mut buf[pos..]) {
            Ok(0) => break,
            Ok(n) => pos += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(pos)
}

fn main() -> io::Result<()> {
    const CHUNK_SIZE: usize = 0x10000;
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();
    let mut in_buf = vec![0; CHUNK_SIZE];
    let mut out_buf = Box::new_uninit_slice(CHUNK_SIZE * 4 + 24);
    let mut aux_buf = Box::new_uninit_slice(CHUNK_SIZE * 2 + 15);
    while let n @ 1.. = read_as_much_as_possible(&mut input, &mut in_buf)? {
        output.write_all(uwuify_into(&in_buf[..n], &mut out_buf, &mut aux_buf))?;
        if n < in_buf.len() {
            break;
        }
    }
    Ok(())
}
