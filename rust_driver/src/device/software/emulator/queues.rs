pub(crate) mod errors;

mod descriptor;

// abstract queue
mod complete_queue;
mod work_queue;

// hardware queue
mod command_request;
mod command_response;
mod meta_report;
mod send;
