mod log_types;

use std::{thread, time::Duration};

use fake::{Fake, Faker};
use log::info;
use log_types::TransportLog;
use serde_json;
use std::io::Write;

use crate::log_types::ExtendedTransportLog;

fn main() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    loop {
        let num = rand::random::<i16>();
        if num % 2 == 0 {
            let log: TransportLog = Faker.fake();
            let str = serde_json::to_string(&log).unwrap();
            info!("{}", str);
        } else {
            let log: ExtendedTransportLog = Faker.fake();
            let str = serde_json::to_string(&log).unwrap();
            info!("{}", str);
        }
        thread::sleep(Duration::from_secs(1))
    }
}
