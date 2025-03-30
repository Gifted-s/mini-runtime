use crate::{Config, worker_pool::Worker};
use crossbeam::channel::{Receiver, Sender, bounded};

pub type TaskSender = Sender<Box<dyn Send + Fn() -> Response>>;
pub type TaskReceiver = Receiver<Box<dyn Send + Fn() -> Response>>;

pub type ResponseSender = Sender<Response>;
pub type ResponseReceiver = Receiver<Response>;

pub struct Runtime {
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
    pub(crate) fn new(config: Config) -> Self {
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

        let runtime = Runtime {
            config,
            task_recevier,
            response_recevier,
            response_sender,
            worker,
            task_sender,
        };
        runtime.worker.start();

        return runtime;
    }

    pub fn schedule<F: Fn() -> Response + Send + 'static>(&self, f: F) {
        match self.task_sender.send(Box::new(f)) {
            Ok(_) => println!("Scheduled"),
            Err(err) => println!("Schedule error {:?}", err),
        }
    }
}
