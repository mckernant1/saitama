use crate::traits::config::Config;
use crossbeam::channel::{Receiver, Sender};

pub trait Orchestrator<C>
where
    C: Config,
{
    fn start(work_send: Sender<bool>, feedback_recv: Receiver<bool>, config: C);
}
