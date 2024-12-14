use std::io;
use std::io::BufWriter;
use std::io::Write;

use serde_json::Map;
use serde_json::Value;

pub fn map2writer<W>(m: Map<String, Value>, w: W) -> Result<(), io::Error>
where
    W: Write,
{
    serde_json::to_writer(w, &m).map_err(io::Error::other)
}

pub fn maps2writer<I, W>(maps: I, mut writer: W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Map<String, Value>, io::Error>>,
    W: Write,
{
    {
        let mut bw = BufWriter::new(&mut writer);
        for r in maps {
            let m: Map<_, _> = r?;
            map2writer(m, &mut bw)?;
            bw.write_all(b"\n")?;
        }
        bw.flush()?;
    }
    writer.flush()
}

pub fn maps2stdout<I>(maps: I) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Map<String, Value>, io::Error>>,
{
    let o = io::stdout();
    let ol = o.lock();
    maps2writer(maps, ol)
}
