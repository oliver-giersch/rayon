extern crate crossbeam_deque;

use crossbeam_deque::Worker;

fn main() {
    let worker = Worker::new_fifo();
    worker.push(1);
}
