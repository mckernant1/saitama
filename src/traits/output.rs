use crate::traits::config::Config;
use crossbeam::channel::Receiver;
use std::fmt::Display;

pub trait OutputHandler<C, R>
where
    R: Display,
    C: Config,
{
    fn handle_output(output_recv: Receiver<Option<R>>, config: C);
}
