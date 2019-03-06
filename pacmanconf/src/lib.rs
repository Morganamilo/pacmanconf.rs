//! # PacmanConf
//!
//! pacmanconf is a simple parser for pacman config files.
//!
//! This parser parses pacman config files textually. It makes no
//! attempt convert fields such as SigLevel or Usage into bitfields.
//!
//! pacmanconf is a wrapper around the pacman-conf binary. Instead
//! of fully parsing pacman.conf files, pacman-conf is used
//! as an intermediate step to greatly simplify the parser logic.
//!
//! However this is more of an implementation detail and pacman-conf
//! will be called automatically when parsing files.

#![warn(missing_docs)]
mod error;
mod pacmanconf;

pub use crate::error::*;
pub use crate::pacmanconf::*;
