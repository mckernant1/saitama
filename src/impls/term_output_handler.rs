use crate::traits::config::Config;
use crate::traits::output::OutputHandler;
use crossbeam::channel::Receiver;
use log::info;
use std::fmt::Display;

pub struct TermOutputHandler;

impl<C, R: Display> OutputHandler<C, R> for TermOutputHandler
where
    C: Config,
{
    fn handle_output(output_recv: Receiver<Option<R>>, _config: C) {
        info!("Starting output handler");
        loop {
            match output_recv.recv().unwrap() {
                Some(request_data) => {
                    println!("{}", request_data)
                }
                None => {
                    break;
                }
            }
        }
    }
}
