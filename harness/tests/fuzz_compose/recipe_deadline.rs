//! Declared timeout signals supervise the external subject without introducing an ambient clock reader.

use std::sync::mpsc::{self, Receiver, RecvTimeoutError, TryRecvError};
use std::time::Duration;

pub(super) fn expired(signal: &Receiver<()>) -> bool {
    match signal.try_recv() {
        Ok(()) | Err(TryRecvError::Disconnected) => true,
        Err(TryRecvError::Empty) => false,
    }
}

pub(super) fn within<T>(
    duration: Duration,
    body: impl FnOnce(&Receiver<()>) -> T,
) -> Result<T, String> {
    std::thread::scope(|scope| {
        let (cancel, cancellation) = mpsc::channel::<()>();
        let (elapsed, signal) = mpsc::channel::<()>();
        let timer = scope.spawn(move || match cancellation.recv_timeout(duration) {
            Err(RecvTimeoutError::Timeout) => elapsed.send(()).map_err(|error| error.to_string()),
            Ok(()) | Err(RecvTimeoutError::Disconnected) => Ok(()),
        });
        let result = body(&signal);
        drop(cancel);
        timer
            .join()
            .map_err(|_unwind| "deadline supervisor timer unwound".to_owned())??;
        Ok(result)
    })
}

#[test]
fn deadline_signals_distinguish_waiting_expired_and_disconnected() -> Result<(), String> {
    let (send, signal) = mpsc::channel();
    assert!(!expired(&signal));
    send.send(()).map_err(|error| error.to_string())?;
    assert!(expired(&signal));
    assert!(!expired(&signal));
    drop(send);
    assert!(expired(&signal));
    within(Duration::ZERO, |expiry| {
        expiry
            .recv_timeout(Duration::from_secs(5))
            .map_err(|error| error.to_string())
    })??;
    assert_eq!(within(Duration::from_secs(30), |_signal| 7_u64)?, 7);
    Ok(())
}
