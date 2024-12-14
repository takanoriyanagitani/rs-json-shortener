use std::env;
use std::io;
use std::process::ExitCode;

use rs_json_shortener::app::stdin2maps2shorter2stdout;
use stdin2maps2shorter2stdout::stdin2maps2shorter2stdout_by_key_source_default;

pub fn key_source_env(env_key: String) -> impl FnMut() -> Result<String, io::Error> {
    move || {
        env::var(env_key.as_str())
            .map_err(|e| io::Error::other(format!("key {env_key} missing: {e}")))
    }
}

pub fn stdin2maps2shorter2stdout(
    env_key: String,
) -> Result<impl FnOnce() -> Result<(), io::Error>, io::Error> {
    let mut key_source = key_source_env(env_key);
    stdin2maps2shorter2stdout_by_key_source_default(&mut key_source)
}

fn sub() -> Result<(), io::Error> {
    let stdin2stdout = stdin2maps2shorter2stdout("ENV_REMOVE_KEY".into())?;
    stdin2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
