//! # scanny
//!
//! `scanny` is a library that makes scanning for open ports more convinient
//!
//! ## Quick Start
//!
//! Here is a quick example usage to print out all open ports
//!
//! ```
//! use scanny::Scanner;
//! #[tokio::main]
//! async fn main() {
//!     let ip = "127.0.0.1".to_string();
//!     let ports = Scanner::scan(ip).await;
//!     println!("Current open ports {:?}"), ports;
//! }
//! ```


use tokio::net::TcpStream;
use tokio::time::{self, Duration};

const MAX_PORT: u16 = 65535;

/// A simple asynchronous port scanner.
///
/// This struct provides functionality to scan all ports (0 to 65535) on a given IP address
/// to determine which ones are open and accepting TCP connections.
pub struct Scanner;

impl Scanner {
    /// Scans through all ports on the specified IP address asynchronously.
    ///
    /// This method attempts to connect to each port within the standard range (0 to 65535)
    /// using a TCP connection. It returns a list of ports that successfully accept the connection,
    /// indicating these ports are open.
    ///
    /// # Parameters
    ///
    /// * `ip`: The IP address to scan, provided as a `String`.
    ///
    /// # Returns
    ///
    /// A `Vec<u16>` containing all open ports found during the scan.
    pub async fn scan(ip: String) -> Vec<u16> {
        let mut ports: Vec<u16> = Vec::new();
        let mut tasks = vec![];

        for port in 0..=MAX_PORT {
            let ip = ip.clone();
            tasks.push(tokio::spawn(async move {
                if Scanner::check_port(&ip, port).await {
                    Some(port)
                } else {
                    None
                }
            }));
        }

        let results = futures::future::join_all(tasks).await;
        for result in results {
            if let Ok(Some(port)) = result {
                ports.push(port);
            }
        }
        ports
    }

    async fn check_port(ip: &str, port: u16) -> bool {
        match time::timeout(Duration::from_secs(1), TcpStream::connect((ip, port))).await {
            Ok(Ok(_)) => true,
            _ => false,
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::runtime::Runtime;

    /// Starts a mock server that listens on the specified port.
    /// This is a helper function used in tests to simulate an open port.
    async fn start_mock_server(port: u16) -> TcpListener {
        TcpListener::bind(("127.0.0.1", port)).await.unwrap()
    }


    /// Tests that the scanner correctly identifies an open port.
    #[test]
    fn test_scan_open_port() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let _listener = start_mock_server(3000).await; // Start a mock server on port 3000
            let ip = "127.0.0.1".to_string();
            let ports = Scanner::scan(ip).await;
            assert!(ports.contains(&3000)); // Check if port 3000 is detected as open
        });
    }

    /// Tests that the scanner does not falsely report closed ports as open
    #[test]
    fn test_scan_closed_port() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let ip = "127.0.0.1".to_string();
            let ports = Scanner::scan(ip).await;
            assert!(!ports.contains(&3001)); // Assuming no server is running on port 3001
        });
    }

}


