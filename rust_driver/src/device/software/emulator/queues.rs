pub(crate) mod errors;

mod descriptor;

// abstract queue
pub(crate) mod complete_queue;
pub(crate) mod work_queue;

// hardware queue
mod command_request;
mod command_response;
mod meta_report;
mod send;

pub(crate) use meta_report::descriptors::*;
