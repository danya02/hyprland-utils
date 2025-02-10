use std::os::unix::ffi::OsStrExt;

pub fn proc_count() -> anyhow::Result<()> {
    let mut old_count = 0;
    let chars = b"09";
    let c0 = chars[0];
    let c9 = chars[1];
    loop {
        let processes = std::fs::read_dir("/proc")?;
        let mut count = 0;
        for process in processes {
            let process = process?;
            if process
                .file_name()
                .as_bytes()
                .iter()
                .all(|c| c >= &c0 && c <= &c9)
            {
                count += 1;
            }
        }

        if count != old_count {
            println!("{}", count);
            old_count = count;
        }

        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
