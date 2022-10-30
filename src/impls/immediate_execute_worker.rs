use crossbeam::channel::{Receiver, Sender};
use crate::traits::config::Config;
use crate::traits::worker::Worker;

pub struct GenericWorker;

pub trait GenericWorkerConfig<R>: Config {

    fn do_load_operation(&self) -> R;

}

impl<C, R> Worker<C, R> for GenericWorker where C: GenericWorkerConfig<R> {
    fn start(
        work_recv: Receiver<bool>,
        output_send: Sender<Option<R>>,
        feedback_send: Sender<bool>,
        config: C
    ) {
        while work_recv.recv().expect("Could not receive from work channel") {
            let output = config.do_load_operation();
            output_send.send(Some(output)).expect("Failed to send output");
            feedback_send.send(true).expect("Failed to send feedback");
        }
    }
}
