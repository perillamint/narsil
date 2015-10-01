extern crate hyper;

use hyper::Client;
use hyper::header::Connection;

pub struct HTTPMonitor {
    monitoring_target: String,
    hyper_client: Client,
}

impl HTTPMonitor {
    fn new(monitoring_target: String) -> HTTPMonitor {
        let mut client = Client::new();
        HTTPMonitor {
            monitoring_target: monitoring_target,
            hyper_client: client,
        }
    }
}
