use crossbeam::channel::{Receiver, Sender};

pub trait Orchestrator<C> {
    fn new(work_send: Sender<bool>, feedback_recv: Receiver<bool>, config: C) -> Self;

    fn start(&self);
}
