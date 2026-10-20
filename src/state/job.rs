use std::time::Duration;


#[derive(Debug)]
pub struct JobState {
    pub job_status: JobStatus,
}

impl Default for JobState {
    fn default() -> Self {
        Self {
            job_status: JobStatus::Pending,
        }
    }
}


impl JobState {

    pub fn start(&mut self) {
        if !matches!(self.job_status, JobStatus::Running(_, _)) {
            let start_time = std::time::Instant::now();
            let (tx, rx) = tokio::sync::oneshot::channel::<Duration>();
            self.job_status = JobStatus::Running(start_time, rx);
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(3)).await;
                let duration = start_time.elapsed();
                let _ = tx.send(duration);
            });
        }
    }

    pub fn tick(&mut self) {
        if let JobStatus::Running(_start_time, rx) = &mut self.job_status {
            if let Ok(duration) = rx.try_recv() {
                self.job_status = JobStatus::Completed(duration);
            }
        }
    }
}

#[derive(Debug)]
pub enum JobStatus {
    Pending,
    Running(std::time::Instant, tokio::sync::oneshot::Receiver<Duration>),
    Completed(std::time::Duration),
}
