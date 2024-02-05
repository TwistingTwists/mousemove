// runtime = exectuor + reactor
// futures = executor + _______  = 1/2 runtime
// https://github.com/richardanaya/executor
// exectuor = only executor

async fn say_hello() {
    println!("async hello");
}

fn main() {
    // say_hello(); // future => executor + ______ ?
    // say_hello().await;
    // await needs executor and reactor (the runtime).
    // you cannot run asynchronous code in sync code. => you need a runtime for it.
    // BUT if you need to run an async code in  a sync code in an async function ?
    //  => need nested runtimes. Nopes. not allowed.
    // async <- sync <- async
}
