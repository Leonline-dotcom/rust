use std::{sync::{mpsc,Arc, Mutex},
          thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}


enum Message{
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        

        for id in 0..size {
           workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{workers,sender}
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() +Send +'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob((job))).unwrap()
    }
}

impl Drop for  ThreadPool{
    fn drop(&mut self) {
        println!("Workers will now go on strike");
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Workers are fired from jobs");

        for Worker in &mut self.workers{
            println!("Shutting down Worker {}", Worker.id);
           if let Some(thread) = Worker.thread.take(){
                thread.join().unwrap()
           }
        }
    }
}

impl Worker{
    fn new(id: usize, receiver:Arc<Mutex< mpsc::Receiver<Message>>>)-> Worker{
        let thread = thread::spawn(move || loop{
        let message = receiver.lock().unwrap().recv().unwrap();
            match message{
                Message::NewJob(job) =>{
                    println!("Worker {} got a job!", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} will be killed", id);
                    break;
                }
            }
       });
       Worker { id, thread:Some(thread) }
    }
}

