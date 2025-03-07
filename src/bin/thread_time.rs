use std::{thread, time::{Duration, SystemTime}};

fn main(){

    show_parallelism();
    println!("thread time!");
    // simple_sleep();
    two_thread_sleep();
}
fn show_parallelism(){
    let para = thread::available_parallelism();

    let msg = match para {
        Ok(number) => { format!("Parallellism {number}")},
        Err(err) => { format!("Error {err}")}
    };
    println!("{}", msg);
}
fn two_thread_sleep(){

    let th = thread::spawn(|| {

        let start = SystemTime::now();
        let sleep_time = Duration::from_millis(400u64);
        loop {

            thread::sleep(sleep_time);
            println!("awake after {:?}", start.elapsed().unwrap_or_default())
        }
    });
    thread::sleep(Duration::from_secs(15u64));
}

fn simple_sleep(){

    const TIMES: i16 = 5;
    for step in 0..TIMES {
        let now = SystemTime::now();
        print!("Step {step} => {now:?}");
        thread::sleep(Duration::from_millis(1500u64));
        let after = SystemTime::now();

        println!(" After {after:?} {:?}", now.elapsed());
    }
}