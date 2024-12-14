use std::io;

use serde_json::Map;
use serde_json::Value;

pub fn map2shorter(remove_key: &str, mut m: Map<String, Value>) -> Map<String, Value> {
    m.remove(remove_key);
    m
}

pub fn maps2shorter<I>(
    remove_key: String,
    maps: I,
) -> impl Iterator<Item = Result<Map<String, Value>, io::Error>>
where
    I: Iterator<Item = Result<Map<String, Value>, io::Error>>,
{
    maps.map(move |r| r.map(|m| map2shorter(&remove_key, m)))
}
