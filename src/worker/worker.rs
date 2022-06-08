use crossbeam::channel::{Receiver, Sender};

pub trait Worker<C, R> {
    fn start(
        work_recv: Receiver<bool>,
        output_send: Sender<Option<R>>,
        feedback_send: Sender<bool>,
        config: C,
    );
}
