/*
Async await (Tasks, Select)
tokio = { version = "1.43.0", features = ["full"] }
*/

#[tokio::main]
async fn main() {
    ex1_simple_async().await;
    ex2_select().await;
}

async fn ex1_simple_async() {
    let mut handles = vec![];
    println!("This code is not part of the async block");

    let s1 = String::from("Huge computation function");
    let s2 = String::from("Simpler computation function");

    let aw1 = tokio::spawn(async move {
        huge_computation(s1).await;
    });
    handles.push(aw1);

    let aw2 = tokio::spawn(async move {
        simple_computation(s2).await;
    });
    handles.push(aw2);

    for handle in handles {
        handle.await.unwrap();
    }
    println!("All tasks have finished");
}

async fn huge_computation(s: String) {
    println!("{:?} has started", s);
    for _i in 0..100_000_000 {}
    println!("{:?} has finished", s);
}

async fn simple_computation(s: String) {
    println!("{:?} has started", s);
    println!("{:?} has finished", s);
}

async fn ex2_select() {
    println!("-- exclusive select block --");
    tokio::select! {
        _ = function1() => println!("Function 1 has finished first"),
        _ = function2() => println!("Function 2 has finished first"),
    }

    println!("-- concurrent select block --");
    // another way to write the select block
    let aw1 = tokio::spawn(async move {
        function1().await
    });
    let aw2 = tokio::spawn(async move {
        function2().await
    });

    tokio::select! {
        _ = aw1 => println!("Function 1 has finished first"),
        _ = aw2 => println!("Function 2 has finished first"),
    }

    println!("-- simple join block --");
    tokio::join!{
        function1(),
        function2(),
    };
}

async fn function1() {
    println!("Function 1 has started");
    for _i in 0..100_000_000 {}
    println!("Function 1 has finished");
}

async fn function2() {
    println!("Function 2 has started");
    println!("Function 2 has finished");
}