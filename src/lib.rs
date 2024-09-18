//! This crate allows you to permanently set environment variables
//!
//! # Examples
//! ```rust
//! // Check if DUMMY is set, if not set it to 1
//! // export DUMMY=1
//! env_perm::check_or_set("DUMMY", 1).expect("Failed to find or set DUMMY");
//! // Append $HOME/some/cool/bin to $PATH
//! // export PATH= "$HOME/some/cool/bin:$PATH"
//! env_perm::append("PATH", "$HOME/some/cool/bin").expect("Couldn't find PATH");
//! // Append $HOME/some/cooler/bin to the front of the path
//! // export PATH="$PATH:$HOME/some/cooler/bin"
//! env_perm::append_to_end("PATH", "$HOME/some/cooler/bin").expect("Couldn't find PATH");
//! // Sets a variable without checking if it exists.
//! // Note you need to use a raw string literal to include ""
//! // export DUMMY="/something"
//! env_perm::set("DUMMY", r#""/something""#).expect("Failed to set DUMMY");
//! ```

use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::env;
use std::fmt;
use dirs;

/// Checks if a environment variable is set.
/// If it is then nothing will happen.
/// If it's not then it will be added
/// to your profile.
pub fn check_or_set<T, U>(var: T, value: U) -> io::Result<()>
where T: fmt::Display + AsRef<std::ffi::OsStr>,
      U: fmt::Display,
{
    env::var(&var)
        .map(|_|())
        .or_else(|_| set(var, value))
}

/// Appends a value to an environment variable
/// Useful for appending a value to PATH
pub fn append<T: fmt::Display>(var: T, value: T) -> io::Result<()> {
    let mut profile = get_profile()?;
    writeln!(profile, "\nexport {}=\"{}:${}\"", var, value, var)?;
    profile.flush()
}

/// Appends a value to an environment variable at either the front or end
pub fn append_to_end<T: fmt::Display>(var: T, value: T) -> io::Result<()> {
    let mut profile = get_profile()?;
    writeln!(profile, "\nexport {}=\"${}:{}\"", var, var, value)?;
    profile.flush()
}

/// Sets an environment variable without checking
/// if it exists.
/// If it does you will end up with two
/// assignments in your profile.
/// It's recommended to use `check_or_set`
/// unless you are certain it doesn't exist.
pub fn set<T: fmt::Display, U: fmt::Display>(var: T, value: U) -> io::Result<()> {
    let mut profile = get_profile()?;
    writeln!(profile, "\nexport {}={}", var, value)?;
    profile.flush()
}

fn get_profile() -> io::Result<File> {
    dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No home directory"))
        .and_then(find_profile)
}

#[cfg(target_family = "unix")]
fn find_profile(mut profile: PathBuf) -> io::Result<File> {
    profile.push(".bash_profile");
    let mut oo = OpenOptions::new();
    oo.append(true)
        .create(false);
    oo.open(profile.clone())
        .or_else(|_|{
            profile.pop();
            profile.push(".bash_login");
            oo.open(profile.clone())
        })
        .or_else(|_|{
            profile.pop();
            profile.push(".profile");
            oo.open(profile.clone())
        })
        .or_else(|_|{
            profile.pop();
            profile.push(".bash_profile");
            oo.create(true);
            oo.open(profile.clone())
        })
}
