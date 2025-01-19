use std::sync::Arc;
use std::time::Instant;
use std::thread;
use ureq::AgentBuilder;

fn main() -> Result<(), ureq::Error> {

    let webpages = vec![
        "https://www.rust-lang.org/",
        "https://doc.rust-lang.org/",
        "https://www.rust-lang.org/learn",
        "https://www.rust-lang.org/tools",
        "https://www.rust-lang.org/community",
    ];

    // Without threads
    let agent = AgentBuilder::new().build();
    let now = Instant::now();

    for web_page in &webpages {
        let _web_body = agent.get(web_page).call()?.into_string()?;
    }
    println!("Time taken without threads: {:.2?}", now.elapsed());

    // With threads
    let now = Instant::now();
    let agent = Arc::new(agent);
    let mut handles: Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new();

    for web_page in webpages {
        let agent = agent.clone();
        let t = thread::spawn(move || {
            let _web_body = agent.get(web_page).call()?.into_string()?;
            Ok(())
        });
        handles.push(t);
    }
    for handle in handles {
        handle.join().unwrap()?;
    }
    println!("Time taken with threads: {:.2?}", now.elapsed());

    Ok(())
}
