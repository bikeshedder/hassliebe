//! Code generators for `hassliebe`.
//!
//! ```text
//! cargo xtask generate <ha-core>   # rewrite all generated regions in src/
//! cargo xtask check <ha-core>      # fail if a region is out of date
//! ```
//!
//! `<ha-core>` is a local checkout of `home-assistant/core` (branch `dev`).

mod generate;
mod platforms;
mod python;
mod region;

use std::{
    error::Error,
    fs,
    io::Write as _,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
};

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<bool, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (write, ha_core) = match args.as_slice() {
        [command, ha_core] if command == "generate" => (true, Path::new(ha_core)),
        [command, ha_core] if command == "check" => (false, Path::new(ha_core)),
        _ => return Err("usage: cargo xtask (generate|check) <ha-core>".into()),
    };
    if !ha_core.join("homeassistant/components/mqtt").is_dir() {
        return Err(format!(
            "{} is not a home-assistant/core checkout",
            ha_core.display()
        )
        .into());
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("xtask has no parent directory")?
        .to_owned();
    let mut up_to_date = true;
    for path in rust_files(&root.join("src"))? {
        let src = fs::read_to_string(&path)?;
        if !src.contains("// @generated begin ") {
            continue;
        }
        let generated = region::replace(&src, |args| generate::region(ha_core, args))
            .map_err(|e| format!("{}: {e}", path.display()))?;
        let formatted = rustfmt(&generated)?;
        if formatted == src {
            continue;
        }
        let relative = path.strip_prefix(&root).unwrap_or(&path).display();
        if write {
            fs::write(&path, formatted)?;
            println!("updated {relative}");
        } else {
            println!("out of date: {relative}");
            up_to_date = false;
        }
    }
    Ok(up_to_date)
}

fn rust_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(rust_files(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn rustfmt(src: &str) -> Result<String, Box<dyn Error>> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or("no stdin")?
        .write_all(src.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "rustfmt failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}
