// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Handler for Unix domain sockets
use super::{Connect, ReadWrite};
use crate::error::{ClientErrorKind, Result};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::Duration;

const DEFAULT_SOCKET_PATH: &str = "/run/parsec/parsec.sock";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

/// IPC handler for Unix domain sockets
#[derive(Debug, Clone)]
pub struct Handler {
    /// Path at which the socket can be found
    path: PathBuf,
    /// Timeout for reads and writes on the streams
    timeout: Option<Duration>,
}

impl Connect for Handler {
    fn connect(&self) -> Result<Box<dyn ReadWrite>> {
        let stream = UnixStream::connect(self.path.clone()).map_err(ClientErrorKind::Ipc)?;

        stream
            .set_read_timeout(self.timeout)
            .map_err(ClientErrorKind::Ipc)?;
        stream
            .set_write_timeout(self.timeout)
            .map_err(ClientErrorKind::Ipc)?;

        Ok(Box::from(stream))
    }

    fn set_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout = timeout;
    }
}

impl Handler {
    /// Create new client using given socket path and timeout duration
    pub fn new(path: PathBuf, timeout: Option<Duration>) -> Self {
        Handler { path, timeout }
    }
}

impl Default for Handler {
    fn default() -> Self {
        Handler {
            path: DEFAULT_SOCKET_PATH.into(),
            timeout: Some(DEFAULT_TIMEOUT),
        }
    }
}
