use assert_cmd::prelude::*;
use std::process::Command;
use structopt::clap::{crate_name, crate_version};

/// Error type used by tests
pub type Error = Box<dyn std::error::Error>;

/// Show help and exit.
#[test]
fn help_shows() -> Result<(), Error> {
    Command::cargo_bin("site24x7_exporter")?
        .arg("--help")
        .assert()
        .success();

    Ok(())
}

/// Show version and exit.
#[test]
fn version_shows() -> Result<(), Error> {
    Command::cargo_bin("site24x7_exporter")?
        .arg("-V")
        .assert()
        .success()
        .stdout(format!("{} {}\n", crate_name!(), crate_version!()));

    Ok(())
}
