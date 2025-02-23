use std::{io::{Error, ErrorKind}, sync::{mpsc::{self, Receiver, SyncSender}, Arc, Mutex}};
use osafe::multiprocessing::posix_thread::PosixThread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub trait Executable
{
    fn try_submit(&self, job: Job) -> Result<(), Error>;
}

#[allow(dead_code)]
struct Worker
{
    id: usize,
    handle: PosixThread<()>
}

#[allow(dead_code)]
pub struct ThreadPool<const J: usize, const N: usize>
{
    sender: SyncSender<Job>,
    threads: Vec<Worker>
}

impl Worker
{
    pub fn new(id: usize, recvr: Arc<Mutex<Receiver<Job>>>) -> Self
    {
        let handle = PosixThread::new(move || loop
        {
            // Get the job. We can unwrap since this is in a new thread
            let job = recvr.lock().unwrap().recv().unwrap();
            // Execute the job
            job();
        }).unwrap();
        // Return the worker
        return Self
        {
            id: id,
            handle: handle
        }
    }
}

impl<const J: usize, const N: usize> ThreadPool<J, N>
{
    pub fn new() -> Self
    {
        // Create the mpsc channel
        let (sender, recvr) = mpsc::sync_channel::<Job>(J);
        // Create the worker vec
        let mut threads = Vec::<Worker>::new();
        let recvr = Arc::new(Mutex::new(recvr));
        for i in 0..N
        {
            // Create the workers
            threads.push(Worker::new(i, Arc::clone(&recvr)));
        }
        // Return the instance
        return Self
        {
            sender: sender,
            threads: threads
        };
    }
}

impl<const J: usize, const N: usize> Executable for ThreadPool<J,N>
{
    fn try_submit(&self, job: Job) -> Result<(), Error>
    {
        self.sender.try_send(job)
        .map_err(|e|
            Error::new(ErrorKind::ResourceBusy, e.to_string())
        )
    }
}


