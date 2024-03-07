use crate::{pacmanconf, Config, Error};

/// The options struct allows you to change settings prior to building.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Options {
    conf_binrary: Option<String>,
    pacman_conf: Option<String>,
    root_dir: Option<String>,
}

impl Config {
    /// Creates a new empty Options instance. This allows you to change parsing options prior to
    /// parsing the config file.
    pub fn options() -> Options {
        Options::new()
    }
}

impl Options {
    /// Creates a new empty Options instance. This allows you to change parsing options prior to
    /// parsing the config file.
    pub fn new() -> Self {
        Default::default()
    }

    /// Configure the path of the pacman-conf helper utility.
    pub fn pacman_conf_bin<S: Into<String>>(&mut self, s: S) -> &mut Self {
        self.conf_binrary = Some(s.into());
        self
    }

    /// Configure the path for the pacman config file. Not setting this
    /// will cause the system default to be used.
    pub fn pacman_conf<S: Into<String>>(&mut self, s: S) -> &mut Self {
        self.pacman_conf = Some(s.into());
        self
    }

    /// Configures pacman's  root directory/
    pub fn root_dir<S: Into<String>>(&mut self, s: S) -> &mut Self {
        self.root_dir = Some(s.into());
        self
    }

    /// Read the config file into a config instance.
    pub fn read(&self) -> Result<Config, Error> {
        pacmanconf::Config::with_opts(
            self.conf_binrary.as_ref(),
            self.pacman_conf.as_ref(),
            self.root_dir.as_ref(),
        )
    }

    /// Expand and dump the config file into a string.
    pub fn expand(&self) -> Result<String, Error> {
        pacmanconf::Config::expand_with_opts(
            self.conf_binrary.as_ref(),
            self.pacman_conf.as_ref(),
            self.root_dir.as_ref(),
        )
    }
}
