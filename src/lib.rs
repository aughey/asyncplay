use std::{pin, time::Duration};

#[allow(dead_code)]
async fn async_foo(f: impl AsyncFn() -> i32) -> i32 {
    f().await
}

// Debounces an async button press
// The async fn is given the button state that will not return
// until it is NOT that state.  It will return the new state of the button.
// (assert: the new state of the button != the old state of the button)
async fn debounce<T>(
    wait_until_not: impl AsyncFn(T) -> T,
    current_value: T,
    steady_duration: std::time::Duration,
    sleep: impl AsyncFn(Duration) -> (),
) where
    T: PartialEq,
{
    loop {
        let new_value = wait_until_not(current_value).await;

        let s = sleep(steady_duration);
        let w = wait_until_not(new_value);
        futures::pin_mut!(s, w);
        futures::select! {
            new_value = w => {
                return new_value;
            }
            () = s => {
                // loop again
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
}
