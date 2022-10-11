use std::{thread, time::Duration};

use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();
    loop {
        let num = rand::random::<i16>();
        if num % 5 == 0 {
            error!("Logging a random number : {}", num);
        } else if num % 5 == 1 {
            warn!("Logging a random number : {}", num);
        } else if num % 5 == 2 {
            info!("Logging a random number : {}", num);
        } else if num % 5 == 3 {
            debug!("Logging a random number : {}", num);
        } else {
            trace!("Logging a random number : {}", num);
        }
        thread::sleep(Duration::from_secs(1))
    }
}
