use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    // NOTE: U: Need to do the same Option take() from Some(val)
    // to move the sender out of ThreadPool. This will allow us
    // to explicitly drop the sender before waiting for the threads
    // to finish.
    sender: Option<mpsc::Sender<Job>>,
}

// Job is a trait object (Box) that holds the type of closure
// that execute will receive.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        // Create a channel to function as the queue of jobs,
        // and execute() will send a job from the ThreadPool
        // to the Worker instances, which will send the job
        // to its thread.
        // Sender of a channel that transmits Job instances.
        let (sender, receiver) = mpsc::channel();

        // NOTE: Arc lets multiple workers own the receiver
        // Mutex ensures only one worker gets a job from
        // the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create some workers and store them in the vector
            // We'll use the receiver in the thread that the workers spawn
            // NOTE: Arc clone to bump the reference count so thew orkers can
            // share ownership of the receiver.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    // TODO: Another approach vs. new(). See Config::build from
    // https://doc.rust-lang.org/book/ch12-00-an-io-project.html
    // pub fn build(size: usize) -> Result<Self, PoolCreationError> {
    //     todo!()
    // }

    // NOTE: We use () with F: FnOnce() because FnOnce represents
    // a closer that takes no parameters (|| ...) and returns the unit type ().
    // NOTE: Send trait needed to transfer the closure to another thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // Use unwrap() in case sending the job down the sending end
        // of the channel fails. It shouldn't but compiler doesn't know.
        // U: Updated to Option so we can drop sender when shutting down
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // U: Move sender out of ThreadPool by taking before
        // we move the threads out of workers to gracefully shutdown
        // NOTE: Dropping the sender closes the channel, which indicates
        // no more messages will be sent. All future sender.recv() calls
        // that the Worker's loop does will return an error
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // NOTE: U: join() takes ownership (consumes) of its arguments
            // Therefore, this moves worker.thread into join. Instead, we need
            // to move the thread out of its Worker instance by using the
            // Option.take() method, which takes the value from Some(val) and
            // replaces with None variant. So, we need to change Worker.thread
            // to be an Option<thread::JoinHandle<()>>.
            // BAD: worker.thread.join().unwrap();
            // With Option<JoinHandle<()>>.
            // NOTE: We use 'if let Some(thread)' to destructure the Some
            // and get the thread. If a worker's thread is already None,
            // we know that worker has already had its thread cleaned up.
            if let Some(thread) = worker.thread.take() {
                // We've taken the inner thread, so now we can join()
                // to ensure all threads complete before shutting down gracefully.
                thread.join().unwrap();
            }
        }
    }
}

// REF: https://doc.rust-lang.org/book/ch20-02-multithreaded.html#a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread
struct Worker {
    id: usize,
    // NOTE: U: join() takes ownership (consumes) of its arguments
    // Therefore, this moves worker.thread into join. Instead, we need
    // to move the thread out of its Worker instance by using the
    // Option.take() method, which takes the value from Some(val) and
    // replaces with None variant. So, we need to change Worker.thread
    // to be an Option<thread::JoinHandle<()>>.
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        // Q: How to create a thread spawned with an empty closure?
        // A: (|| {})
        // NOTE: thread::spawn() will panic, so for production
        // you're better off using thread::Builder and its spawn
        // method that returns Result instead of panicking.
        // NOTE: We'll use the receiver in the thread that the workers spawn
        let thread = thread::spawn(move || loop {
            // U: Gracefully exitthe loop when the ThreadPool drop() implementation
            // calls join() on Worker threads.
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
