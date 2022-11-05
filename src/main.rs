use journald::reader::*;
use std::time::Duration;

fn main() {
    let mut journal =
        JournalReader::open(&JournalReaderConfig::default()).expect("journal open failed");

    // we want a forward walk, there for we have to seek before writing messages
    journal
        .seek(JournalSeek::Head)
        .expect("journal seek failed");

    journal
        .add_filter("_SYSTEMD_UNIT=openvpn.service")
        .expect("Could not set journald filter");

    let mut iter = journal.as_blocking_iter();
    iter.set_timeout(Duration::from_secs(1))
        .expect("Set iter timeout");

    for entry in iter {
        let entry = entry.expect("failed to iterate");

        let entry_message = entry.get_message().unwrap().to_string();
        let entry_time = entry.get_wallclock_time().unwrap().timestamp_us / 1000000;

        println!("{} - {}", entry_time, entry_message)
    }
}
