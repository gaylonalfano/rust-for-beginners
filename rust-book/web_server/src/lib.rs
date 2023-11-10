use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        Self { workers, sender }
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
        self.sender.send(job).unwrap();
    }
}

// REF: https://doc.rust-lang.org/book/ch20-02-multithreaded.html#a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
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
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Self { id, thread }
    }
}
