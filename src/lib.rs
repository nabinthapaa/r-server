use std::{sync::{mpsc::{self, channel}, Arc, Mutex}, thread};


type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in pool
    ///
    /// # panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, reciver) = channel();

        let reciver = Arc::new(Mutex::new(reciver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        };

        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker{
    pub fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>)-> Worker{
        let thread = thread::spawn(move || loop {
            let job = reciver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            println!("Worker {} got a job; Executing.", id);

            job()
        });

        Worker{id, thread}
    }
}














