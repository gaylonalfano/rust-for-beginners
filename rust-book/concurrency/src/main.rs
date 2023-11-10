use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Start MAIN");
    // threads_and_joins();
    // nested_threads_and_move();
    // moving_ownership_to_another_thread();
    // message_passing_threads_and_channels();
    // client_server_multiple_channels_enums();
    // shared_state_multiple_ownership_threads_arc_mutex();
    shared_state_scoped_threads_arc_mutex();
    println!("END of main");
}

fn message_passing_threads_and_channels() {
    let (tx1, rx) = mpsc::channel();

    // Clone our transmitter (sender) so we can send multiple
    // messages from multiple producers (senders) safely.
    let tx2 = tx1.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // NOTE: 'move' transfers ownership of the values it uses
    // from the environment (e.g., main thread), thus transferring
    // ownership of those values from one thread to another.
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn threads_and_joins() {
    // NOTE: Save the JoinHandle into a variable and then wait for
    // the thread to complete by using join()
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from SPAWNED thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // NOTE: This ensures that SPAWNED thread will run to completion,
    // instead of ending prematurely because main thread exits.
    // We block main thread while spawned finishes.
    // NOTE: If we move this above the main thread's for loop,
    // then we'll see spawned complete/print first.
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from MAIN thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn moving_ownership_to_another_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Our sub-thread closure without 'move' will infer/borrow 'v',
    // but the main thread drops 'v' here.
    // drop(v); // oops!

    handle.join().unwrap();
}

fn nested_threads_and_move() {
    let mut n = 1;
    // NOTE: Q: Apparently 'move' here copies 'n' into the closure,
    // so the assignments n = n + 1 inside thread have no effect
    // on outer (main) 'n'. Hmmm... I thought move == move. Hah!
    let t = thread::spawn(move || {
        n += 1;
        println!("Outer: {}", n);
        thread::spawn(move || {
            n += 1;
            println!("Inner: {}", n);
        })
    });
    // Q: Why isn't 'n' 4? What happened to thread 'n'?
    n += 1;
    t.join().unwrap().join().unwrap();
    println!("Final: {}", n); // 2
}

fn client_server_multiple_channels_enums() {
    #[derive(Debug)]
    enum ClientMessage {
        Incr,
        Get,
        Quit,
    }
    #[derive(Debug)]
    enum ServerMessage {
        Get(usize),
    }

    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();
    let server = thread::spawn(move || {
        println!("SPAWNED thread executing...");
        let mut n = 0;
        loop {
            match server_rx.recv().unwrap() {
                ClientMessage::Quit => break,
                ClientMessage::Incr => n += 1,
                ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap(),
            }
        }
    });

    // Now simulate the client sending messages to server
    for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
        println!("client_tx sending msg: {:?}", msg);
        client_tx.send(msg).unwrap();
    }

    // There really is no if condition. Just for printing I believe.
    if let ServerMessage::Get(n) = client_rx.recv().unwrap() {
        println!("n: {}", n);
    }

    // Block main thread to let spawned (server) finish
    server.join().unwrap();
}

fn shared_state_multiple_ownership_threads_arc_mutex() {
    // NOTE: Communicating by shared memory. Multiple threads can
    // access same memory location at the same time.
    // NOTE: Mutexes have two rules (like a panel with one mic scenario):
    // 1. Must attempt to acquire the mutex lock before using the data
    // 2. When done, you must unlock the data so other threads can lock
    // let counter = Arc::new(Mutex::new(0));
    // NOTE: Rc<T> is not a thread-safe type, but Arc<T> is
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // 10
}

// FIXME:
// fn shared_state_arc_multiple_calculation_parts() {
//     // Trying to break up a calculation across multiple threads,
//     // split parts across thread, and then use Mutex<T> to have
//     // each thread update the final result with its part.
//     // Example:
//     // 1. Add up family member ages
//     // 2. Calculate total number of members
//     // 3. Divide total_ages by total_number for final
//     let (tx, rx) = mpsc::channel();
//
//     // Q: Is there where Arc<Mutex<Vec<i32>>> comes in?
//     let member_ages = Arc::new(Mutex::new(vec![9, 10, 12, 42, 42]));
//     let calculated_results = Arc::new(Mutex::new(vec![]));
//     let mut handles = vec![];
//     // Q: What about doing a simple loop for my 3 parts?
//     for calculation in 0..3 {
//         let member_ages = Arc::clone(&member_ages);
//         let t = tx.clone();
//         // FIXME: Q: Do I just push these JoinHandles that each
//         // return a value, but then simply loop over the
//         // handles vector to finish the calculations?
//         // I don't think I need to mix channels + mutex, but could
//         // be wrong...
//         match calculation {
//             0 => {
//                 let result;
//                 handles.push(thread::spawn(move || {
//                     // Compute total ages
//                     result = member_ages.lock().unwrap().iter().sum();
//
//                 }));
//                 results.push(result);
//             }
//             1 => {
//                 handles.push(thread::spawn(move|| {
//                     // Total count of members
//                     member_ages.lock().unwrap().iter().count()
//                 }))
//             }
//             2 => {
//                 handles.push(thread::spawn(move || {
//                     // Compute avg age
//                     members_
//                 }))
//             }
//         }
//         let h = thread::spawn(move || {
//
//         })
//
//
//     }
//     let t2 = tx.clone();
//     let total_ages_handle = thread::spawn(move || {
//         // Acquire mutex lock and then unlock when done
//         let member_ages = Arc::clone(&member_ages);
//         let total_ages: i32 = member_ages.lock().unwrap().iter().sum();
//         t2.send(total_ages).unwrap();
//     });
//
//     // let member_ages = Arc::clone(&member_ages);
//     let t3 = tx.clone();
//     let total_members_handle = thread::spawn(move || {
//         let member_ages = Arc::clone(&member_ages);
//         let total_count: i32 = member_ages.lock().unwrap().iter().count() as i32;
//         t3.send(total_count).unwrap();
//     });
// }

fn shared_state_scoped_threads_arc_mutex() {
    println!("> START fn: Level 0");
    // REF: https://stackoverflow.com/questions/75569769/creating-multiple-mutexes-in-rust-for-thread-synchronization
    let n = 10;

    // NOTE: Using scope() guarantees that all threads spawned inside the scope
    // will be automatically joined before the function returns.
    std::thread::scope(|s| {
        println!(">> Level 1 thread scope");
        let mut shared_list = vec![];
        for i in 0..n {
            println!(">>> Level 2 for. Thread {}", i);
            let shared_data = Arc::new(Mutex::new(0));
            shared_list.push(Arc::clone(&shared_data));

            s.spawn(move || loop {
                println!(">>>> Level 3 spawn loop. Thread {}", i);
                thread::sleep(Duration::from_millis(3));
                // NOTE: First sleep, then lock. Otherwise the lock is held during
                // sleep, blocking whoever wait for the lock.
                let new_data = {
                    let mut data = shared_data.lock().unwrap();
                    *data += 1;
                    *data
                    // NOTE: Release the lock here again, so it isn't held
                    // during println().
                };

                println!("Thread {}: data = {}", i, new_data);

                if new_data == 2 {
                    break;
                }
            });
        }

        loop {
            println!(">>> Level 2 loop");
            thread::sleep(Duration::from_millis(2));
            let sum = shared_list
                .iter()
                .map(|data| *data.lock().unwrap())
                .sum::<i32>();

            println!("shared_data = {}", sum);

            if sum == 10 * 2 {
                break;
            }
        }
        println!("> Level 1 END fn");

        // NOTE: No need for joining - The threads get joined automatically
        // at the end of the scope.
    })
}
