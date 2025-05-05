use sys_info;

fn main() {
    match sys_info::disk_info() {
        Ok(info) => {
            println!("Total disk space: {} KB", info.total);
            println!("Free disk space: {} KB", info.free);
        }
        Err(e) => {
            eprintln!("Failed to get disk information: {}", e);
        }
    }
}
