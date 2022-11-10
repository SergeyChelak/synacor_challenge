/*
Run with command:
    RUST_MIN_STACK=33554432 cargo run

5483          set [r0] 4                           ; r0 = 4
5486          set [r1] 1                           ; r1 = 1
5489         call @6027
6027           jt [r0] @6035                       ; if [r0] != 0 jump 6035
6030          add [r0] [r1]  1                     ; r0 = r1 + 1
6034          ret
6035           jt [r1] @6048                       ; if [r1] != 0 jump 6048
6038          add [r0] [r0]  32767                 ; r0 = (r0 + 32767) % 32768
6042          set [r1] [r7]                        ; reg[1] = reg[7]
6045         call @6027                            ; jump to 6027
6047          ret
6048         push [r0]
6050          add [r1] [r1]  32767                 ; r1 = (r1 + 32767) % 32768
6054         call @6027                            ; jump to 6027
6056          set [r1] [r0]                        ; r1 = r0
6059          pop [r0]
6061          add [r0] [r0]  32767                 ; r0 = r0 + 32767) % 32768
6065         call @6027                            ; jump to 6027
6067          ret
*/
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