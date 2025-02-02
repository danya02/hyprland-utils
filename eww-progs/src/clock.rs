#[tokio::main]
pub async fn clock() -> ! {
    let mut delay_ms;
    loop {
        let current_time = chrono::Local::now();
        let current_millis = current_time.timestamp_millis() % 1000;

        println!("{}", current_time.format("%Y-%m-%d %H:%M:%S"));
        delay_ms = 1000 - current_millis;
        delay_ms = delay_ms.min(1000);

        tokio::time::sleep(tokio::time::Duration::from_millis(
            delay_ms.try_into().unwrap(),
        ))
        .await;
    }
}
