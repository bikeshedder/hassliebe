//! Line-based readers for the few Python constructs the generators need.

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// A member of a `StrEnum` class.
#[derive(Debug, PartialEq)]
pub struct Member {
    /// The Python name, e.g. `KILO_WATT`.
    pub name: String,
    /// The string value, e.g. `kW`.
    pub value: String,
    /// The docstring directly below the member, if any.
    pub docstring: Option<String>,
    /// The `#` comment lines directly above the member, joined, if any.
    pub comment: Option<String>,
}

/// Returns the members of `class <name>(StrEnum):` in declaration order.
pub fn str_enum(src: &str, name: &str) -> Result<Vec<Member>> {
    let header = format!("class {name}(StrEnum):");
    let lines: Vec<&str> = src.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == header)
        .ok_or_else(|| format!("class {name} not found"))?;
    let mut members = Vec::new();
    let mut comment: Vec<&str> = Vec::new();
    let mut i = start + 1;
    while i < lines.len() {
        let line = lines[i];
        if !line.is_empty() && !line.starts_with(' ') {
            break;
        }
        let trimmed = line.trim();
        if let Some(text) = trimmed.strip_prefix('#') {
            comment.push(text.trim());
            i += 1;
            continue;
        }
        if let Some((member, value)) = member_line(line)? {
            let mut docstring = None;
            if let Some(next) = lines.get(i + 1)
                && let Some(rest) = next.trim().strip_prefix("\"\"\"")
            {
                let (text, end) = read_docstring(&lines, i + 1, rest);
                docstring = Some(text);
                i = end;
            }
            members.push(Member {
                name: member,
                value,
                docstring,
                comment: (!comment.is_empty()).then(|| comment.join(" ")),
            });
        }
        comment.clear();
        i += 1;
    }
    if members.is_empty() {
        return Err(format!("class {name} has no members").into());
    }
    Ok(members)
}

/// Returns the names of all top-level `class <prefix>…(StrEnum):` classes.
pub fn str_enum_classes(src: &str, prefix: &str) -> Vec<String> {
    src.lines()
        .filter_map(|line| line.strip_prefix("class "))
        .filter_map(|rest| rest.strip_suffix("(StrEnum):"))
        .filter(|name| name.starts_with(prefix))
        .map(str::to_owned)
        .collect()
}

/// Parses `    NAME = "value"`. Errors on member-like lines with another form.
fn member_line(line: &str) -> Result<Option<(String, String)>> {
    let Some(rest) = line.strip_prefix("    ") else {
        return Ok(None);
    };
    let Some((name, value)) = rest.split_once(" = ") else {
        return Ok(None);
    };
    if name.is_empty()
        || !name
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
    {
        return Ok(None);
    }
    let code = value.split("  #").next().unwrap_or_default().trim();
    let value = string_literal(code).ok_or_else(|| format!("unsupported member line: {line:?}"))?;
    Ok(Some((name.to_owned(), value)))
}

/// Reads a docstring starting at `lines[start]`, whose text after the opening
/// quotes is `rest`. Returns the text and the index of the closing line.
fn read_docstring(lines: &[&str], start: usize, rest: &str) -> (String, usize) {
    if let Some((text, _)) = rest.split_once("\"\"\"") {
        return (text.to_owned(), start);
    }
    let mut text = vec![rest.to_owned()];
    let mut i = start + 1;
    while i < lines.len() {
        if let Some((before, _)) = lines[i].split_once("\"\"\"") {
            text.push(before.to_owned());
            break;
        }
        text.push(lines[i].to_owned());
        i += 1;
    }
    (text.join("\n"), i)
}

/// Returns the entries of a top-level dict literal `NAME = {` (or
/// `NAME: type = {`), as decoded keys and raw value expressions.
pub fn dict(src: &str, name: &str) -> Result<Vec<(String, String)>> {
    let lines: Vec<&str> = src.lines().collect();
    let start = lines
        .iter()
        .position(|line| {
            line.strip_prefix(name).is_some_and(|rest| {
                (rest.starts_with(" =") || rest.starts_with(':')) && rest.ends_with('{')
            })
        })
        .ok_or_else(|| format!("dict {name} not found"))?;
    let mut entries = Vec::new();
    for line in &lines[start + 1..] {
        if line.starts_with('}') {
            return Ok(entries);
        }
        let code = line.split("  #").next().unwrap_or_default().trim();
        if code.is_empty() || code.starts_with('#') {
            continue;
        }
        let entry = code
            .strip_suffix(',')
            .and_then(|entry| {
                let (key, value) = split_key(entry)?;
                Some((string_literal(key)?, value.trim().to_owned()))
            })
            .ok_or_else(|| format!("unsupported dict line in {name}: {line:?}"))?;
        entries.push(entry);
    }
    Err(format!("dict {name} is not closed").into())
}

/// Splits `"key": value` at the colon after the closing quote of the key.
fn split_key(entry: &str) -> Option<(&str, &str)> {
    let rest = entry.strip_prefix('"')?;
    let mut escaped = false;
    for (i, c) in rest.char_indices() {
        match c {
            '\\' if !escaped => escaped = true,
            '"' if !escaped => {
                let key = &entry[..i + 2];
                return rest[i + 1..].strip_prefix(':').map(|value| (key, value));
            }
            _ => escaped = false,
        }
    }
    None
}

/// Decodes a double-quoted Python string literal with `\uXXXX`, `\\` and `\"`
/// escapes. Returns `None` for anything else.
pub fn string_literal(literal: &str) -> Option<String> {
    let inner = literal.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            if c == '"' {
                return None;
            }
            out.push(c);
            continue;
        }
        match chars.next()? {
            'u' => {
                let hex: String = chars.by_ref().take(4).collect();
                out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
            }
            c @ ('\\' | '"') => out.push(c),
            _ => return None,
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = r#"
class Foo(StrEnum):
    """Foo."""

    # Section comment
    A_B = "a_b"
    """First.

    More text.
    """

    # On means on
    C = "c"
    D = "d"  # trailing


class UnitOfBar(StrEnum):
    X = "µx"

ABBREVIATIONS = {
    "a": "alpha",
    "µ": "micro",  # comment
}

TYPED: dict[str, str] = {
    "k": Foo.A_B,
}
"#;

    #[test]
    fn reads_str_enum() {
        let members = str_enum(SRC, "Foo").unwrap();
        assert_eq!(members.len(), 3);
        assert_eq!(members[0].name, "A_B");
        assert_eq!(
            members[0].docstring.as_deref(),
            Some("First.\n\n    More text.\n    ")
        );
        assert_eq!(members[0].comment.as_deref(), Some("Section comment"));
        assert_eq!(members[1].comment.as_deref(), Some("On means on"));
        assert_eq!(members[2].value, "d");
        assert_eq!(str_enum(SRC, "UnitOfBar").unwrap()[0].value, "\u{b5}x");
    }

    #[test]
    fn reads_classes_and_dicts() {
        assert_eq!(str_enum_classes(SRC, "UnitOf"), ["UnitOfBar"]);
        assert_eq!(
            dict(SRC, "ABBREVIATIONS").unwrap(),
            [
                ("a".into(), "\"alpha\"".into()),
                ("\u{b5}".into(), "\"micro\"".into())
            ]
        );
        assert_eq!(
            dict(SRC, "TYPED").unwrap(),
            [("k".into(), "Foo.A_B".into())]
        );
    }

    #[test]
    fn rejects_unknown_forms() {
        assert!(str_enum("class A(StrEnum):\n    X = f\"{Y}\"\n", "A").is_err());
        assert_eq!(string_literal(r#""a\nb""#), None);
    }
}
