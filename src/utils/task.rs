use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

/// “Task” that will eventually hold a value of type `T`.
pub struct Task<T> {
    rx:   Receiver<T>,
    _jh:  JoinHandle<()>,   // kept only to keep the thread alive
}

impl<T> Task<T> {
    /// Spawn `work` on a background thread and return a handle.
    pub fn spawn<F>(work: F) -> Self
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let jh = thread::spawn(move || {
            let value = work();
            // ignore send error (UI might have dropped the Task)
            let _ = tx.send(value);
        });
        Self { rx, _jh: jh }
    }

    /// Non-blocking check; `Some(&T)` once, then always `None`.
    pub fn try_take(&mut self) -> Option<T> {
        use std::sync::mpsc::TryRecvError::*;
        match self.rx.try_recv() {
            Ok(v)           => Some(v),
            Err(Empty)      => None,
            Err(Disconnected) => None,
        }
    }
}
