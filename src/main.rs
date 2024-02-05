use std::time::Duration;
use tokio::task::spawn_blocking;

// runtime = exectuor + reactor
// futures = executor + _______  = 1/2 runtime
// https://github.com/richardanaya/executor
// exectuor = only executor

async fn hello_delay(task: u64, time: u64) {
    println!("Started: {task}");
    let _ = spawn_blocking(move || {
        // tokio::time::sleep(Duration::from_millis(time)).await;
        std::thread::sleep(Duration::from_millis(time));
    })
    .await;
    println!("Finished: {task}");
}

#[tokio::main]
async fn main() {
    // say_hello(); // future => executor + ______ ?
    // say_hello().await;
    // await needs executor and reactor (the runtime).
    // you cannot run asynchronous code in sync code. => you need a runtime for it.
    // BUT if you need to run an async code in  a sync code in an async function ?
    //  => need nested runtimes. Nopes. not allowed.
    // async <- sync <- async

    tokio::join!(
        hello_delay(1, 1200),
        hello_delay(2, 1900),
        hello_delay(3, 2800),
        hello_delay(4, 3900)
    );
}
