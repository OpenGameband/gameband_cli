mod data;

use std::time::{SystemTime, UNIX_EPOCH};

fn pack_time(buf: &mut [u8], offset: usize, seconds: u64) {
        buf[offset] = seconds as u8;
        buf[offset + 1] = (seconds >> 8) as u8;
        buf[offset + 2] = (seconds >> 16) as u8;
        buf[offset + 3] = (seconds >> 24) as u8;
}

fn main() {
    let api = hidapi::HidApi::new().unwrap();

    let (vid, pid) = (0x2a90, 0x0021);
    let device = api.open(vid, pid).unwrap();


    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).expect("It's 1969???").as_secs();
    let mut buf: [u8;9] = [0;9];
    buf[1] = 2;

    pack_time(&mut buf, 5, seconds);

    device.write(&buf).expect("something went wrong");
}
