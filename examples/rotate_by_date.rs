use file_rotate::{
    compression::Compression,
    suffix::{AppendTimestamp, DateFrom, FileLimit},
    ContentLimit, FileRotate, TimeFrequency,
};
use std::io::Write;

use std::thread;
use std::time::Duration;

//sudo mount -t tmpfs -o size=100m tmpfs /mnt/limited_size_dir
//sudo umount /mnt/limited_size_dir  
fn main() {
    let mut log = FileRotate::new(
        "/mnt/limited_size_dir/can_data",
        AppendTimestamp::with_format("%Y%m%d%H%M%S%.3f", FileLimit::Unlimited, DateFrom::FileCreationTime),
        ContentLimit::Bytes(1 * 1024 * 1024),
        Compression::None,
        #[cfg(unix)]
        None,
        10*1024*1024,        
        None,
    );


    for idx in 1..=5 {
        let data: [u8; 1_000_000-1024] = [0; 1_000_000-1024];
        let _ = log.write(&data);
        thread::sleep(Duration::from_secs(1));
    }
}
 