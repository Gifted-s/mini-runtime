use crate::runtime::{Response, ResponseReceiver, ResponseSender, TaskReceiver, TaskSender};
use std::{
    sync::{
        Arc,
        atomic::{AtomicI16, Ordering},
    },
    thread,
    time::Duration,
};

type WorkerCount = u32;

const START_SIGNAL: i16 = -1;
const END_SIGNAL: i16 = 1;

pub struct Worker {
    thread_count: Arc<AtomicI16>,
    task_receiver: TaskReceiver,
    task_sender: TaskSender,
    response_recevier: ResponseReceiver,
    response_sender: ResponseSender,
}

impl Worker {
    pub fn new(
        thread_count: WorkerCount,
        task_receiver: TaskReceiver,
        task_sender: TaskSender,
        response_recevier: ResponseReceiver,
        response_sender: ResponseSender,
    ) -> Self {
        Worker {
            task_receiver,
            thread_count: Arc::new(AtomicI16::new(thread_count as i16)),
            task_sender,
            response_recevier,
            response_sender,
        }
    }

    pub fn start(&self) {
        let mut worker_id = 0;
        for _ in 0..self.thread_count.load(Ordering::Relaxed) {
            worker_id += 1;
            self.worker_runner(worker_id);
        }
    }

    pub fn worker_runner(&self, worker_id: i32) {
        let task_receiver = self.task_receiver.clone();
        let response_sender = self.response_sender.clone();
        let thread_count = self.thread_count.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(500));
                // println!("Recieve from this worker {:?}", worker_id);
                match task_receiver.try_recv() {
                    Ok(task) => {
                        thread_count.fetch_add(START_SIGNAL, Ordering::SeqCst);

                        let output = task();
                        println!("Handled by {:?}", worker_id);
                        match response_sender.send(output) {
                            Ok(sent) => println!("Sent {:?}", sent),
                            Err(err) => println!("Send error {:?}", err),
                        }

                        thread_count.fetch_add(END_SIGNAL, Ordering::SeqCst);
                    }
                    Err(err) => {
                        if err.is_empty() {
                            println!("No messages available. Moving on...");
                        }
                    }
                }
            }
        });
    }
}
