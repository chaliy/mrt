pub struct ProgressReporterContext {
}

pub trait ProgressReporter {
    fn report_output(&self, message: &str);
}