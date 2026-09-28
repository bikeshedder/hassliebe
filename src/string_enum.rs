//! The `string_enum!` macro for string-valued enums with an `Other(String)`
//! fallback.

/// Generates a `#[non_exhaustive]` string enum with an `Other(String)` variant.
///
/// Every variant maps to exactly one wire string and may accept input-only
/// aliases (legacy spellings Home Assistant still understands):
///
/// ```ignore
/// string_enum! {
///     /// Units of measurement.
///     pub enum Unit {
///         /// Micrograms per cubic meter.
///         MicrogramsPerCubicMeter = "μg/m³" | "µg/m³",
///         /// Watt.
///         Watt = "W",
///     }
/// }
/// ```
///
/// The generated enum implements `Serialize`/`Deserialize`, `Display`, `FromStr`
/// (infallible) and has an `as_str()` method. Known strings and aliases map to
/// their variant, everything else to `Other`. Serialization always emits the
/// canonical string. Duplicate strings or aliases fail to compile.
///
/// With a leading `@closed`, the enum has no `Other` variant: it is `Copy`,
/// `as_str()` returns `&'static str`, `FromStr` fails with [`UnknownValue`],
/// deserializing an unknown string is an error, and `ALL` lists all variants.
/// This is meant for enums that are wrapped by an open enum (like the
/// `UnitOf*` classes inside `Unit`).
macro_rules! string_enum {
    (
        @closed
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident = $value:literal $(| $alias:literal)*
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::core::fmt::Debug,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::core::hash::Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #[non_exhaustive]
        $vis enum $name {
            $(
                $(#[$vmeta])*
                #[serde(rename = $value $(, alias = $alias)*)]
                $variant,
            )*
        }

        const _: () = $crate::string_enum::assert_unique(&[$($value, $($alias,)*)*]);

        impl $name {
            /// All variants, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)*];

            /// Returns the string Home Assistant uses for this value.
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)*
                }
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = $crate::UnknownValue;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    $($value $(| $alias)* => Ok(Self::$variant),)*
                    other => Err($crate::UnknownValue(other.into())),
                }
            }
        }
    };
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident = $value:literal $(| $alias:literal)*
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(
            ::core::clone::Clone,
            ::core::fmt::Debug,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::core::hash::Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #[non_exhaustive]
        $vis enum $name {
            $(
                $(#[$vmeta])*
                #[serde(rename = $value $(, alias = $alias)*)]
                $variant,
            )*
            /// A value this crate does not know (yet).
            ///
            /// Use [`FromStr`](::core::str::FromStr) to construct values from
            /// strings: it only yields `Other` for unknown strings.
            #[serde(untagged)]
            Other(::std::string::String),
        }

        const _: () = $crate::string_enum::assert_unique(&[$($value, $($alias,)*)*]);

        impl $name {
            /// Returns the string Home Assistant uses for this value.
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $value,)*
                    Self::Other(value) => value,
                }
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = ::core::convert::Infallible;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                Ok(match s {
                    $($value $(| $alias)* => Self::$variant,)*
                    other => Self::Other(other.into()),
                })
            }
        }
    };
}

pub(crate) use string_enum;

/// Error returned when parsing a string that is not a known value of a closed
/// string enum (such as [`UnitOfPower`](crate::common::unit::UnitOfPower)).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownValue(pub String);

impl std::fmt::Display for UnknownValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown value {:?}", self.0)
    }
}

impl std::error::Error for UnknownValue {}

/// Panics (at compile time when used in a `const`) if `strings` contains a
/// duplicate.
pub(crate) const fn assert_unique(strings: &[&str]) {
    let mut i = 0;
    while i < strings.len() {
        let mut j = i + 1;
        while j < strings.len() {
            if str_eq(strings[i], strings[j]) {
                panic!("string_enum!: duplicate string or alias");
            }
            j += 1;
        }
        i += 1;
    }
}

const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    string_enum! {
        /// Test enum.
        pub enum Fruit {
            /// Apple.
            Apple = "apple",
            /// Micro banana, with a legacy micro sign alias.
            MicroBanana = "μbanana" | "µbanana",
        }
    }

    #[test]
    fn serializes_canonical_string() {
        assert_eq!(serde_json::to_string(&Fruit::Apple).unwrap(), r#""apple""#);
        assert_eq!(
            serde_json::to_string(&Fruit::MicroBanana).unwrap(),
            r#""μbanana""#
        );
    }

    #[test]
    fn deserializes_known_strings_and_aliases() {
        let parse = |s| serde_json::from_str::<Fruit>(s).unwrap();
        assert_eq!(parse(r#""apple""#), Fruit::Apple);
        assert_eq!(parse(r#""μbanana""#), Fruit::MicroBanana);
        assert_eq!(parse(r#""µbanana""#), Fruit::MicroBanana);
    }

    #[test]
    fn unknown_string_round_trips_as_other() {
        let fruit: Fruit = serde_json::from_str(r#""cherry""#).unwrap();
        assert_eq!(fruit, Fruit::Other("cherry".into()));
        assert_eq!(serde_json::to_string(&fruit).unwrap(), r#""cherry""#);
    }

    #[test]
    fn rejects_non_strings() {
        assert!(serde_json::from_str::<Fruit>("1").is_err());
        assert!(serde_json::from_str::<Fruit>("null").is_err());
    }

    #[test]
    fn from_str_agrees_with_serde() {
        for s in ["apple", "μbanana", "µbanana", "cherry"] {
            let json = serde_json::to_string(s).unwrap();
            assert_eq!(
                s.parse::<Fruit>().unwrap(),
                serde_json::from_str::<Fruit>(&json).unwrap()
            );
        }
    }

    #[test]
    fn display_and_as_str() {
        assert_eq!(Fruit::MicroBanana.to_string(), "μbanana");
        assert_eq!(Fruit::Other("cherry".into()).as_str(), "cherry");
    }

    /// `Other` holding a known string serializes like the known variant but is
    /// not equal to it; reading it back normalises it.
    #[test]
    fn other_with_known_string_normalises_on_read() {
        let other = Fruit::Other("apple".into());
        let json = serde_json::to_string(&other).unwrap();
        assert_eq!(json, r#""apple""#);
        assert_ne!(other, Fruit::Apple);
        assert_eq!(serde_json::from_str::<Fruit>(&json).unwrap(), Fruit::Apple);
    }

    string_enum! {
        @closed
        /// Closed test enum.
        pub enum Color {
            /// Red.
            Red = "red",
            /// Grey, also accepting the American spelling.
            Grey = "grey" | "gray",
        }
    }

    #[test]
    fn closed_serde() {
        assert_eq!(serde_json::to_string(&Color::Grey).unwrap(), r#""grey""#);
        assert_eq!(
            serde_json::from_str::<Color>(r#""gray""#).unwrap(),
            Color::Grey
        );
        assert!(serde_json::from_str::<Color>(r#""blue""#).is_err());
    }

    #[test]
    fn closed_from_str() {
        assert_eq!("gray".parse::<Color>(), Ok(Color::Grey));
        assert_eq!(
            "blue".parse::<Color>(),
            Err(crate::UnknownValue("blue".into()))
        );
        assert_eq!(Color::ALL, &[Color::Red, Color::Grey]);
        assert_eq!(Color::Red.to_string(), "red");
    }

    #[test]
    fn assert_unique_accepts_distinct_strings() {
        assert_unique(&["a", "b", "ab"]);
    }

    #[test]
    #[should_panic(expected = "duplicate")]
    fn assert_unique_rejects_duplicates() {
        assert_unique(&["a", "b", "a"]);
    }
}
