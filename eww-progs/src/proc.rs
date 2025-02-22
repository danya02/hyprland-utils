use std::os::unix::ffi::OsStrExt;

use ta::Next;

fn is_numeric(name: &std::ffi::OsStr) -> bool {
    name.as_bytes().iter().all(|c| c.is_ascii_digit())
}

#[derive(serde::Serialize, Default, Debug)]
struct ProcStats {
    count: u64,
    count_1m: f64,
    count_5m: f64,
    count_15m: f64,
}

pub fn proc_count() -> anyhow::Result<()> {
    let mut old_count = 0;
    let duration = std::time::Duration::from_millis(100);
    let intervals_per_1m = 60 * 1000 / duration.as_millis() as usize;
    let intervals_per_5m = 5 * 60 * 1000 / duration.as_millis() as usize;
    let intervals_per_15m = 15 * 60 * 1000 / duration.as_millis() as usize;
    let mut eavg_1m = ta::indicators::ExponentialMovingAverage::new(intervals_per_1m).unwrap();
    let mut eavg_5m = ta::indicators::ExponentialMovingAverage::new(intervals_per_5m).unwrap();
    let mut eavg_15m = ta::indicators::ExponentialMovingAverage::new(intervals_per_15m).unwrap();

    let mut stats = ProcStats::default();

    loop {
        let processes = std::fs::read_dir("/proc")?;
        let mut count = 0;
        for process in processes {
            let process = process?;
            if is_numeric(&process.file_name()) {
                count += 1;
            }
        }

        stats.count_1m = eavg_1m.next(count as f64);
        stats.count_5m = eavg_5m.next(count as f64);
        stats.count_15m = eavg_15m.next(count as f64);
        stats.count = count;

        if count != old_count {
            old_count = count;
            println!("{}", serde_json::to_string(&stats).unwrap());
        }

        std::thread::sleep(duration);
    }
}

#[derive(serde::Serialize, Default, Debug)]
struct ProcRates {
    max_id: u64,
    avg_1m_increase_per_sec: f64,
    avg_5m_increase_per_sec: f64,
    avg_15m_increase_per_sec: f64,
}

pub fn proc_rate() -> anyhow::Result<()> {
    let mut max_id = 0;

    let duration = std::time::Duration::from_millis(100);
    let intervals_per_1m = 60 * 1000 / duration.as_millis() as isize;
    let intervals_per_5m = 5 * 60 * 1000 / duration.as_millis() as isize;
    let intervals_per_15m = 15 * 60 * 1000 / duration.as_millis() as isize;
    let mut buffer = vec![0; intervals_per_15m as usize];
    let mut cursor = 0;
    let len = buffer.len() as isize;

    let wrap = |x: isize| -> usize {
        // Wrap the cursor position to the size of the buffer
        ((x + len) % len) as usize
    };

    loop {
        let processes = std::fs::read_dir("/proc")?;
        for process in processes {
            let process = process?;
            if let Ok(id) = process.file_name().to_str().unwrap().parse::<u64>() {
                max_id = max_id.max(id);
            }
        }

        buffer[cursor] = max_id;
        let icursor = cursor as isize;

        let increase_1m =
            ((buffer[cursor] - buffer[wrap(icursor - intervals_per_1m)]) as f64) / 60.0;
        let increase_5m =
            ((buffer[cursor] - buffer[wrap(icursor - intervals_per_5m)]) as f64) / 300.0;
        let increase_15m =
            ((buffer[cursor] - buffer[wrap(icursor - intervals_per_15m)]) as f64) / 900.0;

        cursor = wrap(icursor + 1);

        let rates = ProcRates {
            max_id,
            avg_1m_increase_per_sec: increase_1m,
            avg_5m_increase_per_sec: increase_5m,
            avg_15m_increase_per_sec: increase_15m,
        };

        println!("{}", serde_json::to_string(&rates).unwrap());

        std::thread::sleep(duration);
    }
}
