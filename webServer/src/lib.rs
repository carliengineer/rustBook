use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

//the closures we’re passing to the thread pool will handle the connection and not return anything, so T will be the unit type () for JoinHandle
pub struct ThreadPool {
    workers : Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

//to differantiate between types of messages received by worker, work start/stop
enum Message {
    NewJob(Job),
    Terminate,
}

//The standard library provides thread::spawn as a way to create threads, and thread::spawn expects to get some code the thread should run as soon as the thread is created. 
//However, in our case, we want to create the threads and have them wait for code that we’ll send later. 
//We’ll implement this behavior by introducing a new data structure between the ThreadPool and the threads that will manage this new behavior. 
//We’ll call this data structure Worker, which is a common term in pooling implementations.
//Each Worker will store a single JoinHandle<()> instance. Then we’ll implement a method on Worker that will take a closure of code to run and send it to the already running thread for execution. 
//We’ll also give each worker an id so we can distinguish between the different workers in the pool when logging or debugging.
//External code (like our server in src/bin/main.rs) doesn’t need to know the implementation details regarding using a Worker struct within ThreadPool, 
//so we make the Worker struct and its new function private.
pub struct Worker {
    id : usize,
    //thread: thread::JoinHandle<()>,
    thread : Option<thread::JoinHandle<()>>,
}

//type decleration
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    //doc comments: 
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(thread_amount : usize) -> ThreadPool {
        assert!(thread_amount > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        //with_capacity function preallocates space in the vector
        let mut workers = Vec::with_capacity(thread_amount);

        for i in 0..thread_amount {
            //single receiver can't be send to man threads  taking a job off the channel queue involves mutating the receiver, so the threads need a safe way to share and modify receiver; otherwise, we might get race conditions.
            workers.push(Worker::new(i, Arc::clone(&receiver)));
            // We want the Worker structs that we just created to fetch code to run from a queue held in the ThreadPool and send that code to its thread to run.
            // We’ll use a channel to function as the queue of jobs, and execute will send a job from the ThreadPool to the Worker instances, which will send the job to its thread
        }

        ThreadPool{workers, sender}
    }
    //we are checing std lib's spawn's lib implementation, so we can see what bounds the signature of that function...
    //we can take closures as parameters with three different traits: Fn, FnMut, FnOnce
    //We still use the () after FnOnce because this FnOnce represents a closure that takes no parameters and returns the unit type ()
    //The F type parameter, that we use to take closures as parameter, has traits bounds defined for it with where keyword.
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

//We use &mut for this because self is a mutable reference, and we also need to be able to mutate worker.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        //The error tells us we can’t call join because we only have a mutable borrow of each worker and join takes ownership of its argument.            //since we can#t give ownership of the worker itself (it is borrowed mutably), we need to move thread out out worker, so thread can be consumed by join
        //currently worker holds to thread::join_handle, if Worker holds an Option<thread::JoinHandle<()>> instead,
        // we can call the take method on the Option to move the value out of the Some variant and leave a None variant in its place.
        // the take() method on Option takes the Some variant out and leaves None in its place. 
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
    
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        //If we used a single loop to iterate through each worker, on the first iteration a terminate message would be sent down the channel and join called on the first worker’s thread. 
        //If that first worker was busy processing a request at that moment, the second worker would pick up the terminate message from the channel and shut down.
        // We would be left waiting on the first worker to shut down, but it never would because the second thread picked up the terminate message. Deadlock!           
    }
}
//we want to create the threads and have them wait for code that we’ll send later
//we do so by genrating empty threads as below in Worker struct
//we need the closure to loop forever, asking the receiving end of the channel for a job and running the job when it gets one.


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}



