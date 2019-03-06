use cini::{Callback, CallbackKind, Ini};
use std::process::Command;
use std::str;
use std::str::FromStr;

use crate::error::{Error, ErrorKind, ErrorLine};

/// A Pacman repository.
///
/// See pacman.conf (5) for information on each field.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Repository {
    /// Name
    pub name: String,
    /// Servers
    pub servers: Vec<String>,
    /// SigLevel
    pub sig_level: Vec<String>,
    /// Usage
    pub usage: Vec<String>,
}

/// A pacman config.
///
/// See pacman.conf (5) for information on each field.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Config {
    /// RootDir
    pub root_dir: String,
    /// DBPath
    pub db_path: String,
    /// CacheDir
    pub cache_dir: Vec<String>,
    /// HookDir
    pub hook_dir: Vec<String>,
    /// GPGDir
    pub gpg_dir: String,
    /// LogFile
    pub log_file: String,
    /// HoldPkg
    pub hold_pkg: Vec<String>,
    /// IgnorePkg
    pub ignore_pkg: Vec<String>,
    /// IgnoreGroup
    pub ignore_group: Vec<String>,
    /// Architecture
    pub architecture: String,
    /// XferCommand
    pub xfer_command: String,
    /// NoUpgrade
    pub no_upgrade: Vec<String>,
    /// NoExtract
    pub no_extract: Vec<String>,
    /// CleanMethod
    pub clean_method: Vec<String>,
    /// SigLevel
    pub sig_level: Vec<String>,
    /// LocalFileSigLevel
    pub local_file_sig_level: Vec<String>,
    /// RemoteFileSigLevel
    pub remote_file_sig_level: Vec<String>,
    /// UseSyslog
    pub use_syslog: bool,
    /// Color
    pub color: bool,
    /// UseDelta
    pub use_delta: f64,
    /// TotalDownload
    pub total_download: bool,
    /// CheckSpace
    pub check_space: bool,
    /// VerpsePkgLists
    pub verbose_pkg_lists: bool,
    /// DisableDownloadTimeout
    pub disable_download_timeout: bool,
    /// ILoveCandy
    pub chomp: bool,
    /// [repo_name]
    pub repos: Vec<Repository>,
}

impl Ini for Config {
    type Err = Error;

    fn callback(&mut self, cb: Callback) -> Result<(), Self::Err> {
        let line = Some(ErrorLine::new(cb.line_number, cb.line));

        match cb.kind {
            CallbackKind::Section(section) => {
                self.handle_section(section);
            }
            CallbackKind::Directive(section, key, value) => {
                self.handle_directive(section, key, value)
                    .map_err(|kind| Error { kind, line })?;
            }
        }

        Ok(())
    }
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = Config::default();
        config.parse_str(s)?;
        Ok(config)
    }
}

impl Config {
    /// Creates a new Config from the default pacman.conf.
    ///
    /// The default pacman.conf location is a compile time option of
    /// pacman but is usually located at /etc/pacman.conf.
    pub fn new() -> Result<Config, Error> {
        Self::with_opts(None, None, None)
    }

    /// Creates a new Config using pacman's compiled in defaults.
    ///
    /// Parsing an empty file causes pacman-conf to fill in each
    /// field with pacman's compiled in default values. This should
    /// not be confused with the `Default::default()` function which
    /// is derived and will give rust's default values eg:
    /// empty string, 0, etc.
    pub fn empty() -> Result<Config, Error> {
        Self::from_file("/dev/null")
    }

    /// Create a new Config from a file.
    pub fn from_file(config: &str) -> Result<Config, Error> {
        Self::with_opts(None, Some(config), None)
    }

    /// Create a new Config with options.
    ///
    /// - bin: The location of the `pacman-conf` binary. Default is
    /// `pacman-conf` in PATH.
    /// - config: Location of config file to parse: Default is
    /// pacman's compiled in default (usually /etc/pacman.conf).
    /// root_dir: The RootDir: Default is pacman's compiled in
    /// default (usually /).
    pub fn with_opts(
        bin: Option<&str>,
        config: Option<&str>,
        root_dir: Option<&str>,
    ) -> Result<Config, Error> {
        let mut cmd = Command::new(bin.unwrap_or("pacman-conf"));
        if let Some(root) = root_dir {
            cmd.args(&["--root", root]);
        }
        if let Some(config) = config {
            cmd.args(&["--config", config]);
        }

        let output = cmd.output()?;

        if !output.status.success() {
            Err(ErrorKind::Runtime(
                String::from_utf8(output.stderr).map_err(|e| e.utf8_error())?,
            ))?;
        }

        let mut config = Config::default();
        config.parse_str(str::from_utf8(&output.stdout)?)?;
        Ok(config)
    }

    fn handle_section(&mut self, section: &str) {
        if section != "options" {
            self.repos.push(Repository {
                name: section.into(),
                ..Default::default()
            });
        }
    }

    fn handle_directive(
        &mut self,
        section: Option<&str>,
        key: &str,
        value: Option<&str>,
    ) -> Result<(), ErrorKind> {
        if let Some(section) = section {
            if section == "options" {
                self.handle_option(section, key, value)
            } else {
                self.handle_repo(section, key, value)
            }
        } else {
            Err(ErrorKind::NoSection(key.into()))
        }
    }

    fn handle_repo(
        &mut self,
        section: &str,
        key: &str,
        value: Option<&str>,
    ) -> Result<(), ErrorKind> {
        let repo = &mut self.repos.iter_mut().last().unwrap();
        let value = value.ok_or_else(|| ErrorKind::MissingValue(section.into(), key.into()));

        match key {
            "Server" => repo.servers.push(value?.into()),
            "SigLevel" => repo.sig_level.push(value?.into()),
            "Usage" => repo.usage.push(value?.into()),
            _ => (),
        }

        Ok(())
    }

    fn handle_option(
        &mut self,
        section: &str,
        key: &str,
        value: Option<&str>,
    ) -> Result<(), ErrorKind> {
        if let Some(value) = value {
            match key {
                "RootDir" => self.root_dir = value.into(),
                "DBPath" => self.db_path = value.into(),
                "CacheDir" => self.cache_dir.push(value.into()),
                "HookDir" => self.hook_dir.push(value.into()),
                "GPGDir" => self.gpg_dir = value.into(),
                "LogFile" => self.log_file = value.into(),
                "HoldPkg" => self.hold_pkg.push(value.into()),
                "IgnorePkg" => self.ignore_pkg.push(value.into()),
                "IgnoreGroup" => self.ignore_group.push(value.into()),
                "Architecture" => self.architecture = value.into(),
                "XferCommand" => self.xfer_command = value.into(),
                "NoUpgrade" => self.no_upgrade.push(value.into()),
                "NoExtract" => self.no_extract.push(value.into()),
                "CleanMethod" => self.clean_method.push(value.into()),
                "SigLevel" => self.sig_level.push(value.into()),
                "LocalFileSigLevel" => self.local_file_sig_level.push(value.into()),
                "RemoteFileSigLevel" => self.remote_file_sig_level.push(value.into()),
                "UseSyslog" => self.use_syslog = true,
                "Color" => self.color = true,
                "UseDelta" => {
                    self.use_delta = value.parse().map_err(|_| {
                        ErrorKind::InvalidValue(section.into(), key.into(), value.into())
                    })?
                }
                _ => (),
            };
        } else {
            match key {
                "TotalDownload" => self.total_download = true,
                "CheckSpace" => self.check_space = true,
                "VerbosePkgLists" => self.verbose_pkg_lists = true,
                "DisableDownloadTimeout" => self.disable_download_timeout = true,
                "UseDelta" => self.use_delta = 0.7,
                "ILoveCandy" => self.chomp = true,
                _ => (),
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_pacman_conf() {
        let pacman_conf = Config {
            root_dir: "/".into(),
            db_path: "/var/lib/pacman/".into(),
            cache_dir: vec!["/var/cache/pacman/pkg/".into()],
            hook_dir: vec!["/etc/pacman.d/hooks/".into()],
            gpg_dir: "/etc/pacman.d/gnupg/".into(),
            log_file: "/var/log/pacman.log".into(),
            hold_pkg: vec!["pacman".into(), "glibc".into()],
            ignore_pkg: vec![
                "linux-ck-headers".into(),
                "linux-ck".into(),
                "vim-youcompleteme*".into(),
                "brackets-bin".into(),
            ],
            ignore_group: vec![],
            architecture: "x86_64".into(),
            xfer_command: "".into(),
            no_upgrade: vec![],
            no_extract: vec![],
            clean_method: vec!["KeepInstalled".into()],
            sig_level: vec![
                "PackageRequired".into(),
                "PackageTrustedOnly".into(),
                "DatabaseOptional".into(),
                "DatabaseTrustedOnly".into(),
            ],
            local_file_sig_level: vec!["PackageOptional".into(), "PackageTrustedOnly".into()],
            remote_file_sig_level: vec!["PackageRequired".into(), "PackageTrustedOnly".into()],
            use_syslog: false,
            color: false,
            use_delta: 0.7,
            total_download: true,
            check_space: true,
            verbose_pkg_lists: true,
            disable_download_timeout: false,
            chomp: true,
            repos: vec![
                Repository {
                    name: "testing".into(),
                    servers: vec![
                        "http://mirror.cyberbits.eu/archlinux/testing/os/x86_64".into(),
                        "https://ftp.halifax.rwth-aachen.de/archlinux/testing/os/x86_64".into(),
                        "https://mirror.cyberbits.eu/archlinux/testing/os/x86_64".into(),
                        "rsync://ftp.halifax.rwth-aachen.de/archlinux/testing/os/x86_64".into(),
                        "http://mirrors.neusoft.edu.cn/archlinux/testing/os/x86_64".into(),
                    ],
                    sig_level: vec![],
                    usage: vec!["All".into()],
                },
                Repository {
                    name: "core".into(),
                    servers: vec![
                        "http://mirror.cyberbits.eu/archlinux/core/os/x86_64".into(),
                        "https://ftp.halifax.rwth-aachen.de/archlinux/core/os/x86_64".into(),
                        "https://mirror.cyberbits.eu/archlinux/core/os/x86_64".into(),
                        "rsync://ftp.halifax.rwth-aachen.de/archlinux/core/os/x86_64".into(),
                        "http://mirrors.neusoft.edu.cn/archlinux/core/os/x86_64".into(),
                    ],
                    sig_level: vec![],
                    usage: vec!["All".into()],
                },
                Repository {
                    name: "extra".into(),
                    servers: vec![
                        "http://mirror.cyberbits.eu/archlinux/extra/os/x86_64".into(),
                        "https://ftp.halifax.rwth-aachen.de/archlinux/extra/os/x86_64".into(),
                        "https://mirror.cyberbits.eu/archlinux/extra/os/x86_64".into(),
                        "rsync://ftp.halifax.rwth-aachen.de/archlinux/extra/os/x86_64".into(),
                        "http://mirrors.neusoft.edu.cn/archlinux/extra/os/x86_64".into(),
                    ],
                    sig_level: vec![],
                    usage: vec!["All".into()],
                },
                Repository {
                    name: "community-testing".into(),
                    servers: vec![
                        "http://mirror.cyberbits.eu/archlinux/community-testing/os/x86_64".into(),
                        "https://ftp.halifax.rwth-aachen.de/archlinux/community-testing/os/x86_64"
                            .into(),
                        "https://mirror.cyberbits.eu/archlinux/community-testing/os/x86_64".into(),
                        "rsync://ftp.halifax.rwth-aachen.de/archlinux/community-testing/os/x86_64"
                            .into(),
                        "http://mirrors.neusoft.edu.cn/archlinux/community-testing/os/x86_64"
                            .into(),
                    ],
                    sig_level: vec![],
                    usage: vec!["All".into()],
                },
                Repository {
                    name: "community".into(),
                    servers: vec![
                        "http://mirror.cyberbits.eu/archlinux/community/os/x86_64".into(),
                        "https://ftp.halifax.rwth-aachen.de/archlinux/community/os/x86_64".into(),
                        "https://mirror.cyberbits.eu/archlinux/community/os/x86_64".into(),
                        "rsync://ftp.halifax.rwth-aachen.de/archlinux/community/os/x86_64".into(),
                        "http://mirrors.neusoft.edu.cn/archlinux/community/os/x86_64".into(),
                    ],
                    sig_level: vec![],
                    usage: vec!["All".into()],
                },
                Repository {
                    name: "multilib-testing".into(),
                    servers: vec![
                        "http://mirror.cyberbits.eu/archlinux/multilib-testing/os/x86_64".into(),
                        "https://ftp.halifax.rwth-aachen.de/archlinux/multilib-testing/os/x86_64"
                            .into(),
                        "https://mirror.cyberbits.eu/archlinux/multilib-testing/os/x86_64".into(),
                        "rsync://ftp.halifax.rwth-aachen.de/archlinux/multilib-testing/os/x86_64"
                            .into(),
                        "http://mirrors.neusoft.edu.cn/archlinux/multilib-testing/os/x86_64".into(),
                    ],
                    sig_level: vec![],
                    usage: vec!["All".into()],
                },
                Repository {
                    name: "multilib".into(),
                    servers: vec![
                        "http://mirror.cyberbits.eu/archlinux/multilib/os/x86_64".into(),
                        "https://ftp.halifax.rwth-aachen.de/archlinux/multilib/os/x86_64".into(),
                        "https://mirror.cyberbits.eu/archlinux/multilib/os/x86_64".into(),
                        "rsync://ftp.halifax.rwth-aachen.de/archlinux/multilib/os/x86_64".into(),
                        "http://mirrors.neusoft.edu.cn/archlinux/multilib/os/x86_64".into(),
                    ],
                    sig_level: vec![],
                    usage: vec!["All".into()],
                },
            ],
        };

        assert_eq!(pacman_conf, Config::from_file("tests/pacman.conf").unwrap());
    }

    #[test]
    fn test_success() {
        Config::new().unwrap();
        Config::empty().unwrap();
        Config::with_opts(None, None, None).unwrap();
        Config::with_opts(None, Some("tests/pacman.conf"), None).unwrap();
        Config::from_file("tests/pacman.conf").unwrap();
    }

    #[test]
    fn test_error() {
        let err = Config::from_str(
            "
                                    [options]
                                    Color
                                    [repo]
                                    Server
                                    ",
        )
        .unwrap_err();

        if let ErrorKind::MissingValue(s, k) = err.kind {
            assert_eq!(s, "repo");
            assert_eq!(k, "Server");
            assert_eq!(err.line.unwrap().number, 5);
        } else {
            panic!("Error kind is not MissingValue");
        }
    }
}
