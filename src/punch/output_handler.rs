use crossbeam::channel::{Receiver, RecvError};
use log::info;
use crate::punch::request_record::RequestRecord;

pub fn handle_output(
    output_recv: Receiver<Option<RequestRecord>>
) {
    info!("Starting output handler");
    loop {
        match output_recv.recv().unwrap() {
            Some(request_data) => {
                println!("{}", request_data.to_csv_string())
            }
            None => {
                break;
            }
        }
    }
}
