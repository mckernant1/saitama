use crossbeam::channel::Receiver;
use std::fmt::Display;

pub trait OutputHandler<R>
where
    R: Display,
{
    fn handle_output(output_recv: Receiver<Option<R>>);
}
