// init_logging is in examples
use std::time::{Duration, Instant};

async fn mysleep(i: u32) -> u32 {
    let r = rand::random::<u64>();
    let wait = r % 500;
    tokio::time::sleep(Duration::from_secs(1) + Duration::from_millis(wait)).await;
    println!("{}", i);
    i * 2
}

async fn test_f_join_all(iter: impl Iterator<Item = u32>) -> Vec<u32> {
    use futures::future;

    future::join_all(iter.map(|i| mysleep(i) )).await
}

async fn test_f_stream(iter: impl Iterator<Item = u32>) -> Vec<u32> {
    use futures::stream::{self, StreamExt};

    stream::iter(iter).map(|i| mysleep(i)).buffered(64).collect().await
    //stream::iter(iter).map(|i| mysleep(i)).collect().await
}

async fn test_f_lite_stream(iter: impl Iterator<Item = u32>) -> Vec<u32> {
    use futures_lite::stream::{self, StreamExt};

    stream::iter(iter).map(|i| mysleep(i)).collect().await
    //stream::iter(iter).collect().await
}

#[tokio::main]
async fn main() {
    let v = (1..10).map(|i| i);
    let start = Instant::now();
    let r: Vec<_> = test_f_join_all(v.clone()).await;
    println!("{:?}", r);
    let duration = start.elapsed();
    println!("test_f_join_all: {:?}", duration);

    let start = Instant::now();
    println!("{:?}", test_f_stream(v.clone()).await);
    let duration = start.elapsed();
    println!("test_f_stream: {:?}", duration);

    let start = Instant::now();
    println!("{:?}", test_f_lite_stream(v.clone()).await);
    let duration = start.elapsed();
    println!("test_f_lite_stream: {:?}", duration);
}

