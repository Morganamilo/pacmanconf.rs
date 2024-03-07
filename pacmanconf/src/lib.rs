//! # PacmanConf
//!
//! pacmanconf is a simple parser for pacman config files.
//!
//! ```
//! use pacmanconf::Config;
//!
//! # fn main() {
//! let config = Config::new().expect("failed to parse config");
//!
//! let config = Config::options()
//!     .root_dir("/chroot")
//!     .pacman_conf("tests/pacman.conf")
//!     .read()
//!     .expect("failed to parse config");
//!
//!     for repo in &config.repos {
//!         println!("{}", repo.name);
//!     }
//! # }
//! ```
//!
//! See [`Config`] and [`Options`] on how to use this library.

#![warn(missing_docs)]
mod error;
mod options;
mod pacmanconf;

pub use crate::error::*;
pub use crate::options::*;
pub use crate::pacmanconf::*;
