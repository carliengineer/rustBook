use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::rc::{Rc};



// Concurrent programming, where different parts of a program execute independently
// Parallel programming, where different parts of a program execute at the same time
fn main() {
    // to create a new thread, we call the thread::spawn function and pass it a closure
    //We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of thread::spawn in a variable
    //The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish
    //By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    let threadHandle = thread::spawn(||
        {
            for i in 1..10 {
                println!("Hi number {} from the SPAWNED thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }
    );

    for i in 1..5 {
        println!("hi number {} from the MAIN thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //to make sure threads will run and finish
    threadHandle.join().unwrap();
    //Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
    //so if you put this before main thread, firs 10 SPAWNED threads will execute, then main thread will start.

    // The move closure is often used alongside thread::spawn because it allows you to use data from one thread in another thread.
    // By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    let v = vec![1, 2, 3];
    let handler = thread::spawn(move ||
        {
            println!("Here's a vector we captured from main thread's env: {:?}", v);
        }
    );
    //following can't be used as move above already moved
    //drop(v);
    handler.join().unwrap();

    //Channels
    //mpsc stands for multiple producer, single consumer
    // transmitter tx, receiver rx
    let (tx, rx) = mpsc::channel();

    let channelHandler = thread::spawn(move ||
        {
            let val = String::from("gokhan");
            tx.send(val).unwrap();
        }
    );

    //recv or try_recv
    //Once a value is sent, recv will return it in a Result<T, E>. When the sending end of the channel closes, recv will return an error to signal that no more values will be coming.
    //The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time.
    let receivedStr = rx.recv().unwrap();
    println!("Got this: {}", receivedStr);
    //Using try_recv is useful if this thread has other work to do while waiting for messages: we could write a loop that calls try_recv every so often,
    // handles a message if one is available, and otherwise does other work for a little while until checking again

    let (tx1,rx1) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move ||
        {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        }
    );

    thread::spawn(move ||
        {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx2.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        }
    );
    //this keeps running as long as channel is open
    for receivedMsg in rx1 {
        println!("Received: {}", receivedMsg);
    }


    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        //After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside.
        *num = 6;
        // lock release happens automatically. when num goes outof scope
    }

    println!("m: {:?}", m);

    //multi-thread mutex : with multiple owners
    //Rc<T> is not safe to share across threads.  so can't be used t ohold mutex.
    //Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. The a stands for atomic, meaning it’s an atomically reference counted type.
    //atomics work like primitive types but are safe to share across threads.thread safety comes with a performance penalty that you only want to pay when you really need to.
    //If you’re just performing operations on values within a single thread, your code can run faster if it doesn’t have to enforce the guarantees atomics provide.
    //counter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does
    let counter =Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move||
            {
                let mut counterNum = counter.lock().unwrap();
                *counterNum += 1;
            }
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    //Mutex<T> comes with the risk of creating deadlocks.
    //These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.

    println!("counter: {:?}", counter);


    //sync trait
    // The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
    // In other words, any type T is Sync if &T (a reference to T) is Send, meaning the reference can be sent safely to another thread.
    // Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.
    // Almost every Rust type is Send, but there are some exceptions, including Rc<T>: this cannot be Send
    // because if you cloned an Rc<T> value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time.
    // For this reason, Rc<T> is implemented for use in single-threaded situations where you don’t want to pay the thread-safe performance penalty.




}

