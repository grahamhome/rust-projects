use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::mpsc;
    use super::*;

    #[test]

    /// Test showing that threads spawned from the main thread will be shut down when the
    /// main thread stops. Only the main thread will finish counting in this example.
    fn spawn() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread", i);
                thread::sleep(Duration::from_millis(1))
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn spawn_and_join() {
        use super::*;
        let join_handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
        // Block the main thread until the spawned thread completes.
        join_handle.join().unwrap();
    }

    #[test]
    fn moving() {
        let v = vec![1,2,3];
        let handle = thread::spawn(move || {
            println!("here's a vector: {:?}", v);
        });
        handle.join().unwrap();

    }

    #[test]
    fn channels() {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("Hello from spawned thread");
            // Thread owns tx
            // send() returns a Result because the receiver may or may not still be valid
            tx.send(val).unwrap();
        });

        // recv() blocks until a message is received, unlike try_recv()
        let received = rx.recv().unwrap();
        println!("Got \"{}\"", received);
    }

    #[test]
    fn more_messages() {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hello"),
                String::from("from"),
                String::from("spawned"),
                String::from("thread"),
            ];

            // Iteration ends when channel is closed
           for val in vals {
               tx.send(val).unwrap();
               thread::sleep(Duration::from_secs(1))
           }
        });

        for received in rx {
            println!("{}", received)
        }
    }

    #[test]
    fn multi_producer() {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hello"),
                String::from("from"),
                String::from("first"),
                String::from("thread"),
            ];

            // Iteration ends when channel is closed
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("hello"),
                String::from("from"),
                String::from("second"),
                String::from("thread"),
            ];

            // Iteration ends when channel is closed
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        });

        for received in rx {
            println!("{}", received)
        }
    }

    #[test]
    fn mutex() {
        use std::sync::Mutex;

        let m = Mutex::new(5);

        {
            // Mutex::lock() returns a LockResult because another thread may panic
            // while holding the mutex
            let mut num = m.lock().unwrap();
            // LockResult::unwrap() returns a MutexGuard, a smart pointer implementing Deref.
            // Also implements Drop to release mutex automatically when going out of scope.
            *num = 6;
        }
        println!("m = {:?}", m);
    }

    #[test]
    fn mutex_sharing() {
        // Arc = Atomic Rc, safe for multithreading
        use std::sync::{Arc, Mutex};

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }

}