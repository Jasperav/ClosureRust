use std::sync::{Mutex, Arc};
use ws::{Handler, connect};
use std::thread::sleep;
use std::time::Duration;

struct TestClient<'a>(&'a Mutex<bool>);

impl Handler for TestClient<'_> { }

fn main() {
    let is_closed = Arc::new(Mutex::new(false));

    std::thread::spawn( {
        let a = Arc::clone(&is_closed);
        move || connect("127.0.0.1", |c| TestClient(&*a));
    });

    sleep(Duration::from_millis(10_000));

    assert_eq!(*is_closed.lock().unwrap(), true);
}