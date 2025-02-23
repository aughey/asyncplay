use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let wait_until_not = move |old_state: i32| {
        async move {
            if old_state <= 10 {
                old_state + 1
            } else {
                // Wait forever
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    };
    let sleep = || async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    };

    let result = asyncplay::debounce(wait_until_not, 0, sleep).await;
    assert_eq!(result, 11);
    Ok(())
}
