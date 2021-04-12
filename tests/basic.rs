mod utils;

use assert_cmd::prelude::*;
use std::io::Read;
use std::process::Command;
use structopt::clap::{crate_name, crate_version};
use utils::{Error, TestProcess};

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

// TODO
// #[test]
// fn serves_geolocations() -> Result<(), Error> {
//     let mut dh = TestProcess::new(vec![""])?;
//
//     reqwest::blocking::get(&dh.url)?.error_for_status()?;
//
//     dh.child.kill()?;
//     let mut output = String::new();
//     dh.child
//         .stdout
//         .as_mut()
//         .unwrap()
//         .read_to_string(&mut output)?;
//
//     assert!(output.is_empty());
//
//     Ok(())
// }
