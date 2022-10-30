use crate::traits::config::Config;
use crossbeam::channel::{Receiver, Sender};

pub trait Worker<C, R>
where
    C: Config,
{
    fn start(
        work_recv: Receiver<bool>,
        output_send: Sender<Option<R>>,
        feedback_send: Sender<bool>,
        config: C,
    );
}
