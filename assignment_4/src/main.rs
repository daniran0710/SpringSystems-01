use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;

    let (tx, rx) = mpsc::channel::<i32>();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // Producers
    for id in 1..=2 {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT / 2);
        }));
    }

    // Consumers
    for id in 1..=3 {
        let rx_clone = Arc::clone(&rx);
        handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Send termination signals
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let value = rng.gen_range(1..=100);
        println!("Producer {id} produced {value}");
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    println!("Producer {id} finished producing.");
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {id} received termination signal. Exiting.");
            break;
        }

        println!("Consumer {id} processed value {value}");
        thread::sleep(Duration::from_millis(150));
    }
}
