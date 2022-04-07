use std::sync::{
    atomic::{self, Ordering},
    Arc,
};
use tokio::time;

#[tokio::main]
async fn main() {
    let stopped = Arc::new(atomic::AtomicBool::new(false));

    let stop_flag = Arc::clone(&stopped);
    let handle = tokio::spawn(async move {
        while (!stop_flag.load(Ordering::Relaxed)) {
            tokio::time::sleep(time::Duration::from_secs(1)).await;
            println!("Ping");
        }
    });

    let terminate_task = tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        stopped.store(true, Ordering::Relaxed);
    });

    terminate_task.await;
}
