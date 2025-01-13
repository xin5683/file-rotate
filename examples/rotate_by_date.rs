use file_rotate::{
    compression::Compression,
    suffix::{AppendTimestamp, DateFrom, FileLimit},
    ContentLimit, FileRotate, TimeFrequency,
};
use std::io::Write;

fn main() {
    let mut log = FileRotate::new(
        "/mnt/limited_size_dir/can_data",
        AppendTimestamp::with_format("%Y%m%d%H%M%S", FileLimit::Unlimited, DateFrom::DateYesterday),
        ContentLimit::Bytes(1 * 1024 * 1024),
        Compression::None,
        #[cfg(unix)]
        None,
    );


    for idx in 1..=5 {
        let data: [u8; 1_000_000-1024] = [0; 1_000_000-1024];
        let _ = log.write(&data);
    }
}
 