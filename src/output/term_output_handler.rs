use std::fmt::Display;
use crate::output::output::OutputHandler;
use crossbeam::channel::Receiver;
use log::info;

pub struct TermOutputHandler;

impl <T: Display> OutputHandler<T> for TermOutputHandler {
    fn handle_output(output_recv: Receiver<Option<T>>) {
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
