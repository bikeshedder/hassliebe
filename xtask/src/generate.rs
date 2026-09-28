//! The generators behind each region kind.

use std::{error::Error, fmt::Write as _, fs, path::Path};

use crate::{platforms, python};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Reads a file of the Home Assistant core checkout.
fn read(ha_core: &Path, path: &str) -> Result<String> {
    fs::read_to_string(ha_core.join(path)).map_err(|e| format!("{path}: {e}").into())
}

/// Dispatches a region to its generator.
pub fn region(ha_core: &Path, args: &[&str]) -> Result<String> {
    match args {
        ["unit-classes"] => unit_classes(ha_core),
        ["unit-wrapper"] => unit_wrapper(ha_core),
        ["device-class", path, class] => device_class(ha_core, path, class),
        ["abbreviations", dict] => abbreviations(ha_core, dict),
        ["component", platform] => platforms::render(platform),
        _ => Err(format!("unknown region {args:?}").into()),
    }
}

/// `KILO_WATT_HOUR` → `KiloWattHour`.
fn camel(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                first
                    .to_uppercase()
                    .chain(chars.flat_map(char::to_lowercase))
                    .collect()
            })
        })
        .collect()
}

/// `BloodGlucoseConcentration` → `blood glucose concentration`.
fn words(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push(' ');
        }
        out.extend(c.to_lowercase());
    }
    out
}

const CONST_PY: &str = "homeassistant/const.py";
const SENSOR_CONST_PY: &str = "homeassistant/components/sensor/const.py";

fn unit_class_names(ha_core: &Path) -> Result<Vec<String>> {
    let mut classes = python::str_enum_classes(&read(ha_core, CONST_PY)?, "UnitOf");
    classes.sort();
    Ok(classes)
}

/// One closed `string_enum!` per `UnitOf*` class, with the legacy spellings
/// from `AMBIGUOUS_UNITS` as aliases.
fn unit_classes(ha_core: &Path) -> Result<String> {
    let src = read(ha_core, CONST_PY)?;
    let ambiguous = python::dict(&read(ha_core, SENSOR_CONST_PY)?, "AMBIGUOUS_UNITS")?;
    let mut used = 0;
    let mut out = String::new();
    for class in unit_class_names(ha_core)? {
        let short = &class["UnitOf".len()..];
        writeln!(out, "string_enum! {{")?;
        writeln!(out, "    @closed")?;
        writeln!(
            out,
            "    /// Units of {}, like `{class}` in Home Assistant.",
            words(short)
        )?;
        writeln!(out, "    ///")?;
        writeln!(out, "    /// Source: `{CONST_PY}`")?;
        writeln!(out, "    pub enum {class} {{")?;
        for member in python::str_enum(&src, &class)? {
            let reference = format!("{class}.{}", member.name);
            let alias = ambiguous
                .iter()
                .find(|(_, value)| *value == reference)
                .map(|(key, _)| key);
            let (note, alias_suffix) = match alias {
                Some(alias) => {
                    used += 1;
                    (
                        format!(" Also accepts `{alias}` when reading."),
                        format!(" | \"{alias}\""),
                    )
                }
                None => (String::new(), String::new()),
            };
            writeln!(
                out,
                "        /// `{}` (`{}`).{note}",
                member.value, member.name
            )?;
            writeln!(
                out,
                "        {} = \"{}\"{alias_suffix},",
                camel(&member.name),
                member.value
            )?;
        }
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out)?;
    }
    let expected = ambiguous
        .iter()
        .filter(|(_, value)| value.starts_with("UnitOf"))
        .count();
    if used != expected {
        return Err(format!("{used} of {expected} AMBIGUOUS_UNITS aliases matched a unit").into());
    }
    Ok(out.trim_end().to_owned() + "\n")
}

/// `Power(UnitOfPower),` for every class, in the precedence order (sorted).
fn unit_wrapper(ha_core: &Path) -> Result<String> {
    Ok(unit_class_names(ha_core)?
        .iter()
        .map(|class| format!("{}({class}),\n", &class["UnitOf".len()..]))
        .collect())
}

/// The variants of a device class (or similar) `StrEnum`, documented from the
/// docstring, or from the `#` comments if every member without a docstring has
/// one.
fn device_class(ha_core: &Path, path: &str, class: &str) -> Result<String> {
    let members = python::str_enum(&read(ha_core, path)?, class)?;
    let use_comments = members
        .iter()
        .filter(|member| member.docstring.is_none())
        .all(|member| member.comment.is_some());
    let mut out = String::new();
    for member in &members {
        let lines: Vec<&str> = member
            .docstring
            .as_deref()
            .map(|doc| doc.trim().lines().map(str::trim).collect())
            .unwrap_or_default();
        let mut title = match lines.first() {
            Some(first) => (*first).to_owned(),
            None => capitalize(&member.value.replace('_', " ")),
        };
        if !title.ends_with('.') {
            title.push('.');
        }
        writeln!(out, "/// {title}")?;
        if member.docstring.is_none()
            && use_comments
            && let Some(comment) = &member.comment
        {
            let period = if comment.ends_with('.') { "" } else { "." };
            writeln!(out, "///\n/// {comment}{period}")?;
        }
        if let Some(units) = units_paragraph(&lines)
            && units != "Unit of measurement: `None`"
        {
            writeln!(out, "///\n/// {units}")?;
        }
        writeln!(out, "{} = \"{}\",", camel(&member.name), member.value)?;
    }
    Ok(out)
}

/// `carbon monoxide` → `Carbon monoxide`.
fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first
            .to_uppercase()
            .chain(chars.flat_map(char::to_lowercase))
            .collect()
    })
}

/// The "Unit of measurement:" paragraph of a sensor device class docstring,
/// joined into one line.
fn units_paragraph(lines: &[&str]) -> Option<String> {
    let start = lines
        .iter()
        .position(|line| line.starts_with("Unit of measurement:"))?;
    let mut units = String::new();
    for (i, line) in lines[start..].iter().enumerate() {
        if line.is_empty() {
            break;
        }
        if i == 0 {
            units.push_str(line);
        } else if let Some(item) = line.strip_prefix("- ") {
            units.push_str(if units.ends_with(':') { " " } else { "; " });
            units.push_str(item.trim());
        } else {
            units.push(' ');
            units.push_str(line);
        }
    }
    Some(units.trim_end_matches(',').to_owned())
}

/// `("abbreviation", "full"),` for every entry, sorted by abbreviation.
fn abbreviations(ha_core: &Path, dict: &str) -> Result<String> {
    let src = read(ha_core, "homeassistant/components/mqtt/abbreviations.py")?;
    let mut entries = python::dict(&src, dict)?
        .into_iter()
        .map(|(key, value)| {
            python::string_literal(&value)
                .map(|value| (key, value))
                .ok_or_else(|| format!("{dict}: value is not a string: {value}"))
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    entries.sort();
    Ok(entries
        .iter()
        .map(|(key, value)| format!("(\"{key}\", \"{value}\"),\n"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(camel("KILO_WATT_HOUR_PER_100_KM"), "KiloWattHourPer100Km");
        assert_eq!(camel("PM25"), "Pm25");
        assert_eq!(
            words("BloodGlucoseConcentration"),
            "blood glucose concentration"
        );
        assert_eq!(capitalize("carbon monoxide"), "Carbon monoxide");
    }

    #[test]
    fn units_paragraph_joins_lists() {
        let lines = [
            "Water.",
            "",
            "Unit of measurement:",
            "- SI / metric: `m³`, `L`",
            "- USCS / imperial: `ft³`",
        ];
        assert_eq!(
            units_paragraph(&lines).as_deref(),
            Some("Unit of measurement: SI / metric: `m³`, `L`; USCS / imperial: `ft³`")
        );
    }
}
