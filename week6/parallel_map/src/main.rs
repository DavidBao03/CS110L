use crossbeam_channel;
use std::{thread, time};

struct MsgIn<T> {
    val: T,
    idx: usize,
}

struct MsgOut<U> {
    val: U,
    idx: usize,
}

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let size = input_vec.len();
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    output_vec.resize_with(input_vec.len(), Default::default);
    let mut threads = Vec::with_capacity(num_threads);
    let (out_sender, out_reciver) = crossbeam_channel::unbounded::<MsgOut<U>>();
    let (in_sender, in_reciver) = crossbeam_channel::unbounded::<MsgIn<T>>();
    // implement parallel map
    for _ in  0..num_threads {
        let in_reciver = in_reciver.clone();
        let out_sender = out_sender.clone();
        threads.push(thread::spawn(move || {
            while let Ok(msg) = in_reciver.recv() {
                out_sender.send(MsgOut {val: f(msg.val), idx: msg.idx}).expect("Sending fail!");
            }
        }));
    }

    for i in 0..size {
        let val = input_vec.pop().unwrap();
        let msg = MsgIn {val: val, idx: size - i - 1,};
        in_sender.send(msg).expect("Sending fail!");
    }

    drop(out_sender);

    drop(in_sender);

    while let Ok(msg) = out_reciver.recv() {
        output_vec[msg.idx] = msg.val;
    }

    for thread in threads {
        thread.join().expect("Panic in current thread");
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
