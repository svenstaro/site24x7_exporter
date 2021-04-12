use assert_cmd::prelude::*;
use port_check::{free_local_port, is_port_reachable};
use std::ffi::OsStr;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Error type used by tests
pub type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct TestProcess {
    pub child: Child,
    pub port: String,
    pub url: String,
}

impl Drop for TestProcess {
    fn drop(&mut self) {
        if let Err(e) = self.child.kill() {
            eprintln!("WARN: {}", e);
        }
    }
}

#[allow(dead_code)]
impl TestProcess {
    /// Get a Dummyhttp instance on a free port.
    pub fn new<I, S>(args: I) -> Result<TestProcess, Error>
    where
        I: IntoIterator<Item = S> + Clone + std::fmt::Debug,
        S: AsRef<OsStr> + PartialEq + From<&'static str>,
    {
        let port = free_local_port()
            .expect("Couldn't find a free local port")
            .to_string();

        let child = Command::cargo_bin("site24x7_exporter")?
            .arg("--web.listen-address")
            .arg(format!("0.0.0.0:{}", port))
            .args(args.clone())
            .stdout(Stdio::piped())
            .spawn()?;

        // Wait a max of 1s for the port to become available.
        let start_wait = Instant::now();
        while start_wait.elapsed().as_secs() < 1
            && !is_port_reachable(format!("localhost:{}", port))
        {
            sleep(Duration::from_millis(100));
        }

        let url = format!("http://localhost:{port}", port = port);

        Ok(TestProcess { child, port, url })
    }
}
