use std::thread;

use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(async move {
        loop {
            tx.send("Future 1".to_string()).await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });

    let handler = worker(rx);
    handler.join().unwrap();
    // tokio::task::spawn_blocking(move || {
    //     while let Some(s) = rx.blocking_recv() {
    //         let ret = expensive_blocking_task(s);
    //         println!("result: {}", ret);
    //     }
    // });

    Ok(())
}

fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Some(s) = rx.blocking_recv() {
            let ret = expensive_blocking_task(s);
            println!("result: {}", ret);
        }
    })
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(std::time::Duration::from_secs(1));
    blake3::hash(s.as_bytes()).to_string()
}
