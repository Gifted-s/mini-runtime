use crate::{Config, worker_pool::Worker};
use crossbeam::channel::{Receiver, Sender, bounded};
use std::sync::{Arc, Mutex, mpsc};

pub type TaskSender = Sender<Box<dyn Send + Fn() -> Response>>;
pub type TaskReceiver = Receiver<Box<dyn Send + Fn() -> Response>>;

pub type ResponseSender = Sender<Response>;
pub type ResponseReceiver = Receiver<Response>;

struct Runtime {
    config: Config,

    task_sender: TaskSender,
    task_recevier: TaskReceiver,

    response_sender: ResponseSender,
    response_recevier: ResponseReceiver,

    worker: Worker,
}
pub struct Response {}

//struct Request {}

impl Runtime {
    fn new(config: Config) -> Self {
        let (task_sender, task_recevier) = bounded(config.resource.io.io_queue_size);

        let (response_sender, response_recevier) =
            bounded(config.resource.response.response_queue_size);

        let worker = Worker::new(
            config.resource.worker_thread_count,
            task_recevier.clone(),
            task_sender.clone(),
            response_recevier.clone(),
            response_sender.clone(),
        );

        Runtime {
            config,
            task_recevier,
            response_recevier,
            response_sender,
            worker,
            task_sender,
        }
    }

    fn start<F: Fn()>(f: F) {
        //
    }
}
