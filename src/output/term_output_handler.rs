use crate::model::request_record::RequestRecord;
use crate::output::output::OutputHandler;
use crossbeam::channel::Receiver;
use log::info;

pub struct TermOutputHandler;

impl OutputHandler<RequestRecord> for TermOutputHandler {
    fn handle_output(output_recv: Receiver<Option<RequestRecord>>) {
        info!("Starting output handler");
        loop {
            match output_recv.recv().unwrap() {
                Some(request_data) => {
                    println!("{}", String::from(request_data))
                }
                None => {
                    break;
                }
            }
        }
    }
}
