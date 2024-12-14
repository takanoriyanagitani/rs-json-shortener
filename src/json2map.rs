use std::io;

use serde_json::Map;
use serde_json::Value;

pub fn json2map(j: &[u8]) -> Result<Map<String, Value>, io::Error> {
    serde_json::from_slice(j).map_err(io::Error::other)
}

pub fn jsons2maps<I>(jsons: I) -> impl Iterator<Item = Result<Map<String, Value>, io::Error>>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
{
    jsons.map(|r| r.and_then(|b| json2map(&b)))
}
