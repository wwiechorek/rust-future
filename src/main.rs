use tokio::time::sleep;
use futures::join;
use std::time::Duration;

async fn wait_for(time: u64) -> u64 {
    let seconds = Duration::new(time, 0);
    sleep(seconds).await;
    println!("Wait for {} second(s)", time);
    time
}

async fn start() {
    let t1a = wait_for(2).await;
    println!("Done: {}", t1a);
    let t2a = wait_for(1).await;
    println!("Done: {}", t2a);

    let t1 = wait_for(2);
    let t2 = wait_for(1);
    let (r1, r2) = join!(t1, t2);

    println!("Done: {} and {}", r1, r2);
}

#[tokio::main]
async fn main() {
    println!("Start");
    start().await;
    println!("Done");
}