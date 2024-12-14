use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub fn reader2lines<R>(rdr: R, line: u8) -> impl Iterator<Item = Result<Vec<u8>, io::Error>>
where
    R: Read,
{
    let br = BufReader::new(rdr);
    br.split(line)
}

pub fn stdin2lines(line: u8) -> impl Iterator<Item = Result<Vec<u8>, io::Error>> {
    let i = io::stdin();
    let il = i.lock();
    reader2lines(il, line)
}
