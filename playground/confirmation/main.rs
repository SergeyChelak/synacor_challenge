// mod alg;
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

fn f(a: u16, b: u16, r7: u16, memo: &mut HashMap<(u16, u16), u16>) -> u16 {
    if let Some(result) = memo.get(&(a, b)) {
        return *result;
    }
    let mut r0 = a;
    let mut r1 = b;
    while r0 != 0 {
        r1 = if r1 > 0 {
            f(r0, r1 - 1, r7, memo)
        } else {
            r7
        };
        r0 -= 1;
    }
    let result = (r1 + 1) % 32768;
    memo.insert((a, b), result);
    result
}

fn main() {
    let threads_number = 4;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 1..=threads_number {
        let handle = thread::spawn(move || {
            let mut memo: HashMap<(u16, u16), u16> = HashMap::new();
             for r7 in (i..=32767).step_by(threads_number) {
                memo.clear();
                let r0 = f(4, 1, r7 as u16, &mut memo);
                if r0 == 6 {
                    println!("r0 = {r0}, r7 = {r7}");
                }
                // println!("r0 = {r0}, r7 = {r7}");
            }
            println!("Thread #{i} completed");
        });
        handles.push(handle);
    }
    for h in handles {
        _ = h.join();
    }
    println!("Done");
}