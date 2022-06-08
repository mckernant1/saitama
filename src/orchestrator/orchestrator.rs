use crossbeam::channel::{Receiver, Sender};

pub trait Orchestrator<C> {
    fn start(work_send: Sender<bool>, feedback_recv: Receiver<bool>, config: C);
}
