use crossbeam::channel::Receiver;

pub trait OutputHandler<R> {
    fn handle_output(output_recv: Receiver<Option<R>>);
}
