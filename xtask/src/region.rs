//! Generated regions in source files:
//!
//! ```text
//! // @generated begin <kind> [args…]
//! …replaced on every run…
//! // @generated end
//! ```

use std::error::Error;

const BEGIN: &str = "// @generated begin ";
const END: &str = "// @generated end";

/// Replaces every region in `src` with `generate(kind_and_args)`. The
/// generated text is indented like the begin marker.
pub fn replace(
    src: &str,
    mut generate: impl FnMut(&[&str]) -> Result<String, Box<dyn Error>>,
) -> Result<String, Box<dyn Error>> {
    let mut out = String::new();
    let mut lines = src.lines().enumerate();
    while let Some((number, line)) = lines.next() {
        out.push_str(line);
        out.push('\n');
        let Some(spec) = line.trim_start().strip_prefix(BEGIN) else {
            continue;
        };
        let indent = &line[..line.len() - line.trim_start().len()];
        let args: Vec<&str> = spec.split_whitespace().collect();
        let generated = generate(&args).map_err(|e| format!("line {}: {spec}: {e}", number + 1))?;
        let end = loop {
            match lines.next() {
                Some((_, line)) if line.trim_start() == END => break line,
                Some((_, line)) if line.trim_start().starts_with(BEGIN) => {
                    return Err(format!("line {}: nested region", number + 1).into());
                }
                Some(_) => {}
                None => return Err(format!("line {}: region not closed", number + 1).into()),
            }
        };
        for generated_line in generated.lines() {
            if !generated_line.is_empty() {
                out.push_str(indent);
                out.push_str(generated_line);
            }
            out.push('\n');
        }
        out.push_str(end);
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_and_indents() {
        let src = "a\n    // @generated begin x 1\n    old\n    // @generated end\nb\n";
        let out = replace(src, |args| {
            assert_eq!(args, ["x", "1"]);
            Ok("new\n\nlines\n".into())
        })
        .unwrap();
        assert_eq!(
            out,
            "a\n    // @generated begin x 1\n    new\n\n    lines\n    // @generated end\nb\n"
        );
    }

    #[test]
    fn unclosed_region() {
        assert!(replace("// @generated begin x\n", |_| Ok(String::new())).is_err());
    }
}
