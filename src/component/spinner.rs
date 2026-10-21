const JOB_SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn spinner_text(millis: u128) -> &'static str {
    JOB_SPINNER[(millis / 100) as usize % JOB_SPINNER.len()]
}

pub fn spinner_text_from_start(started: std::time::Instant) -> &'static str {
    spinner_text(started.elapsed().as_millis())
}