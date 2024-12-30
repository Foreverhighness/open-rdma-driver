pub mod errors;

mod descriptor;

// abstract queue
pub mod complete_queue;
pub mod work_queue;

// hardware queue
mod command_request;
mod command_response;
mod meta_report;
mod send;

pub use meta_report::descriptors::*;
