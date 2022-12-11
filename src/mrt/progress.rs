use log::info;

pub struct LogProgressReporter {
}

impl ProgressReporter for LogProgressReporter {
    fn report_output(&self, message: &str) {
        info!("{}", message);
    }
}

pub trait ProgressReporter {
    fn report_output(&self, message: &str);
}