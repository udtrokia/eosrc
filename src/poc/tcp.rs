use std::thread;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use super::vm::Vm;

/// consts
pub const TCP_PACKAGE_SIZE: usize = 512;

/// SJ TcpServer
pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub fn new(addr: &'static str) -> TcpServer {
        TcpServer {
            listener: TcpListener::bind(addr).unwrap(),
        }
    }

    pub fn serve<F>(self, handler: F) -> std::io::Result<()>
    where F: Fn(TcpStream, Arc<Vm>) + Send + Sync + 'static + Copy {
        println!("SpaceJam launch at {}...", self.listener.local_addr().unwrap());

        let pool = ThreadPool::new(4);
        let vm = Arc::new(Vm::default());
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let vm = vm.clone();
            pool.execute(move|| {
                handler(stream, vm);
            });
        }

        Ok(())
    }

    pub fn send(self, addr: &'static str, content: &'static str) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write(content.as_bytes())?;

        let mut data = [0; TCP_PACKAGE_SIZE];
        stream.read(&mut data)?;

        let text = String::from_utf8(data.to_vec()).unwrap();
        println!("[Client] received: {}", text);

        Ok(())
    }
}

/// Store Closures
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

/// Signals
enum Message {
    NewJob(Job),
    Terminate,
}

/// mpsc ThreadPool
/// Ever a job come in, free thread will lock and work on it.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// Thread Worker
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        job.call_box();
                    },
                    Message::Terminate => {
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
