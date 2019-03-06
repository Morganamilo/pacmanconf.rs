//! # cini
//!
//! cini is a small **c**allback based **ini** parser framework.
//!
//! This crate provides a simple way to implement an ini parser.
//! Unlike many other ini parsers which are map based solutions,
//! cini parses inis into structs via the [Ini](trait.Ini.html)
//! trait. Although to do this the struct must manually implement
//! [Ini](trait.Ini.html) (a custom derive is probably possible
//! but out of scope for me).
//!
//! As this crate was originally created for parsing pacman's
//! pacman.conf, the ini format exactly follows pacman's.

#![warn(missing_docs)]

/// The kind of callback.
pub enum CallbackKind<'a> {
    /// A new section has been declared. This variant contains
    /// the section name.
    Section(&'a str),
    /// A new directive has been devlared. This variant contains:
    ///
    /// - The current section (if any)
    /// - The key of the directive
    /// - The value of the directive (if any)
    Directive(Option<&'a str>, &'a str, Option<&'a str>),
}

/// The callback implemnters of [Ini](trait.Ini.html) receive for each
/// line parsed.
pub struct Callback<'a> {
    /// The filename of the current ini file (if any)
    pub filename: Option<&'a str>,
    /// The current line that has been parsed
    pub line: &'a str,
    /// The line number of the current line
    pub line_number: usize,
    /// The kind of line parsed
    pub kind: CallbackKind<'a>,
}

/// Parse an ini str into a struct.
///
/// # Example
///
/// ```rust
/// use cini::{Callback, CallbackKind, Ini};
///
/// #[derive(Default)]
/// struct Config {
///     foo: i32,
///     bar: i32,
///     cake: bool,
/// }
///
/// impl Ini for Config {
///     type Err = String;
///
///     fn callback(&mut self, cb: Callback) -> Result<(), Self::Err> {
///         match cb.kind {
///             CallbackKind::Section(section) => Err("No sections allowed".to_string()),
///             CallbackKind::Directive(section, key, value) => {
///                 match key {
///                     "foo" => self.foo = value.unwrap().parse().unwrap(),
///                     "bar" => self.bar = value.unwrap().parse().unwrap(),
///                     "cake" => self.cake = true,
///                     _ => return Err(format!("Unknown key: {}", key)),
///                 }
///                 Ok(())
///             }
///         }
///     }
/// }
///
/// fn main() {
///     let ini = "
///         foo = 5
///         bar = 44
///         cake
///     ";
///
///     let mut config = Config::default();
///     config.parse_str(ini).unwrap();
///
///     assert_eq!(config.foo, 5);
///     assert_eq!(config.bar, 44);
///     assert_eq!(config.cake, true);
/// }
/// ```
pub trait Ini {
    /// The associated error which can be returned from parsing.
    type Err;

    /// The callback function that is called for every line parsed.
    fn callback(&mut self, cb: Callback) -> Result<(), Self::Err>;

    /// Parses an ini str into a struct.
    ///
    /// This function takes the struct via `&mut self`. This means
    /// many different ini files could be parsed by calling this
    /// method repeatidly.
    fn parse_str(&mut self, ini: &str) -> Result<(), Self::Err> {
        self.parse(None, ini)
    }

    /// Parses an ini str into a struct. Optionally a filename can be
    /// supplied, this is passed to the callback so that error
    /// messages can contain the filename.
    ///
    /// Note this method still reads from a str. You must write
    /// your own code to open a file and pass it to this method
    ///
    /// This function takes the struct via `&mut self`. This means
    /// many different ini files could be parsed by calling this
    /// method repeatidly.
    fn parse(&mut self, filename: Option<&str>, ini: &str) -> Result<(), Self::Err> {
        let mut section = None;

        for (line_number, line) in ini.lines().enumerate() {
            let line = line.trim();
            let kind;
            let line_number = line_number + 1;

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                let header = &line[1..line.len() - 1];
                kind = CallbackKind::Section(header);
                section = Some(header);
            } else {
                let pair = split_pair(line);
                kind = CallbackKind::Directive(section, pair.0, pair.1)
            }

            let data = Callback {
                filename,
                line,
                line_number,
                kind,
            };

            self.callback(data)?;
        }

        Ok(())
    }
}

fn split_pair(s: &str) -> (&str, Option<&str>) {
    let mut split = s.splitn(2, '=');
    (
        split.next().unwrap().trim_end(),
        split.next().map(|s| s.trim_start()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[derive(Default)]
    struct Config {
        cake: bool,
        amount: u32,
        lie: bool,
    }

    impl Ini for Config {
        type Err = String;

        fn callback(&mut self, cb: Callback) -> Result<(), Self::Err> {
            match cb.kind {
                CallbackKind::Section(section) => assert_eq!(section, "nom"),
                CallbackKind::Directive(section, key, value) => {
                    assert_eq!(section, Some("nom"));
                    match key {
                        "cake" => self.cake = true,
                        "amount" => self.amount = value.unwrap().parse().unwrap(),
                        "lie" => self.lie = value.unwrap().parse().unwrap(),
                        _ => panic!("that's not cake"),
                    }
                }
            }

            Ok(())
        }
    }

    impl FromStr for Config {
        type Err = <Config as Ini>::Err;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut config = Config::default();
            config.parse_str(s).map(|_| config)
        }
    }

    #[test]
    fn cake() {
        let ini = "
        [nom]
        cake
        amount = 23
        lie = true
        #comment";
        let config: Config = ini.parse().unwrap();
        assert_eq!(config.cake, true);
        assert_eq!(config.amount, 23);
        assert_eq!(config.lie, true);
    }

    #[test]
    fn comment() {
        let mut config = Config::default();
        config.parse_str("#cake").unwrap();
        assert_eq!(config.cake, false);
    }

    #[test]
    #[should_panic]
    fn no_cake() {
        let mut config = Config::default();
        config
            .parse_str(
                "[nom]
                         not a cake",
            )
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn no_section() {
        let mut config = Config::default();
        config.parse_str("cake").unwrap();
    }
}
