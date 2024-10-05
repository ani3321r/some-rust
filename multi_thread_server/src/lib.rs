use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool{
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message{
  NewJob(Job),
  Terminate,
}

impl ThreadPool{
  pub fn new(size: usize) -> ThreadPool{
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size{
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool {workers, sender}
  }

  pub fn execute<F>(&self, f: F)
  where 
    F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }  
}

impl Drop for ThreadPool{
  fn drop(&mut self){
    println!("sending terminate message to all workers");
    for _ in &self.workers{
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("shutting down all workers");

    for worker in &mut self.workers{
      println!("shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take(){
        thread.join().unwrap();
      }
    }
  }
}

struct Worker{
  id: usize,
  thread: Option<thread::JoinHandle<()>>
}

impl Worker{
  fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
    let thread = thread::spawn(move || loop{
      let message = receiver
        .lock()
        .unwrap()
        .recv()
        .unwrap();

        match message{
          Message::NewJob(job) => {
            println!("Worker {} got a job executing", id);
            job();
          }
          Message::Terminate => {
            println!("worker {} was told to terminate", id);
            break;
          }
        }
    });

    Worker{id, thread: Some(thread)}
  }
}

/*
getting output 😁
worker 0 got a job executing
worker 1 got a job executing
worker 2 got a job executing
*/


// after implimenting shut down and cleanup for the server, the output
/*
Worker 0 got a job executing
sending terminate message to all workers
shutting down all workers
shutting down worker 0
worker 2 was told to terminate
worker 3 was told to terminate
Worker 1 got a job executing
worker 0 was told to terminate
shutting down worker 1
worker 1 was told to terminate
shutting down worker 2
shutting down worker 3
*/