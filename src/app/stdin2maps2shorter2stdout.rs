use std::io;

use serde_json::Map;
use serde_json::Value;

use crate::compose;
use crate::curry;

use crate::rdr2lines::stdin2lines;

use crate::json2map::jsons2maps;

use crate::maps2shorter::maps2shorter;

use crate::maps2writer::maps2stdout;

pub fn stdin2maps(line: u8) -> impl Iterator<Item = Result<Map<String, Value>, io::Error>> {
    compose!(stdin2lines, jsons2maps)(line)
}

pub fn maps2shorter2stdout<I>(remove_key: String, maps: I) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Map<String, Value>, io::Error>>,
{
    compose!(curry!(maps2shorter)(remove_key), maps2stdout)(maps)
}

pub const LINE_DEFUALT: u8 = b'\n';

pub struct Config {
    pub line: u8,
    pub remove_key: String,
}

impl Config {
    pub fn new(line: u8, remove_key: String) -> Self {
        Self { line, remove_key }
    }

    pub fn new_default(remove_key: String) -> Self {
        Self::new(LINE_DEFUALT, remove_key)
    }

    pub fn new_by_key_source_default<K>(key_source: &mut K) -> Result<Self, io::Error>
    where
        K: FnMut() -> Result<String, io::Error>,
    {
        let remove_key: String = key_source()?;
        Ok(Self::new_default(remove_key))
    }
}

pub fn config2stdin2maps2shorter2stdout(cfg: Config) -> impl FnOnce() -> Result<(), io::Error> {
    move || compose!(stdin2maps, curry!(maps2shorter2stdout)(cfg.remove_key))(cfg.line)
}

pub fn stdin2maps2shorter2stdout_by_key_source_default<K>(
    key_source: &mut K,
) -> Result<impl FnOnce() -> Result<(), io::Error>, io::Error>
where
    K: FnMut() -> Result<String, io::Error>,
{
    let cfg: Config = Config::new_by_key_source_default(key_source)?;
    Ok(move || compose!(stdin2maps, curry!(maps2shorter2stdout)(cfg.remove_key))(cfg.line))
}
