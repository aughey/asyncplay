#[allow(dead_code)]
async fn async_foo(f: impl AsyncFn() -> i32) -> i32 {
    f().await
}

/// Debounces an async "button" press.  This doesn't have to be an actual button, just something that
/// logically has an unpressed and a not-unpressed state.  The transition of unpressed and not unpressed
/// can be noisy, so this function will wait until the state is stable before returning the new state.
///
/// Stability is defined as the stable_wait future completing while the state is in the not-unpressed state.

/// wait_until_not: a function that takes the current state of the button and returns the new state once it no longer current state.
/// current_value: the current state of the button which is passed to the wait_until_not function.
/// sleep: Provides an async wait that, if it expires, means that the value is stable.
pub async fn debounce<T, WAIT, SLEEP>(
    wait_until_not: WAIT,
    not_pressed_value: T,
    stable_wait: SLEEP,
) -> T
where
    T: PartialEq + Clone,
    WAIT: AsyncFn(T) -> T,
    SLEEP: AsyncFn() -> (),
{
    loop {
        let new_value = wait_until_not(not_pressed_value.clone()).await;

        let s = stable_wait();
        let w = wait_until_not(new_value.clone());
        futures::pin_mut!(s, w);
        match futures::future::select(s, w).await {
            futures::future::Either::Left((_, _)) => {
                // The sleep finished first, so we return the new value
                return new_value;
            }
            futures::future::Either::Right((_, _)) => {
                // The wait finished first, so we continue the loop
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!("Hello", "Hello");
    }

    #[tokio::test]
    async fn test_async_foo() {
        let f = || async { 42 };
        let result = async_foo(f).await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_debounce() {
        let count = std::cell::Cell::new(0);
        let wait_until_not = |_old_state: i32| {
            async {
                if count.get() <= 10 {
                    count.replace(count.get() + 1);
                    count.get()
                } else {
                    // Wait forever
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        };
        let stable_time = || async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        };

        let result = debounce(wait_until_not, 0, stable_time).await;
        assert_eq!(result, 11);
    }
}
