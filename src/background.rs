use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

use eframe::egui;

use crate::app::R2GlassApp;
use crate::history::HistoryEntry;
use crate::r2_session::R2Error;
use crate::visual::decompiler_crashed;

pub(crate) type JobResult = Result<String, R2Error>;

pub(crate) struct BackgroundJob {
    label: String,
    command: String,
    refresh_after: bool,
    fallback_command: Option<String>,
    receiver: Receiver<JobResult>,
    poll_count: u64,
}

impl BackgroundJob {
    fn new(
        label: impl Into<String>,
        command: impl Into<String>,
        refresh_after: bool,
        fallback_command: Option<String>,
        receiver: Receiver<JobResult>,
    ) -> Self {
        Self {
            label: label.into(),
            command: command.into(),
            refresh_after,
            fallback_command,
            receiver,
            poll_count: 0,
        }
    }

    fn poll_interval(&self) -> Duration {
        // Exponential backoff: 100ms, 200ms, 400ms, 800ms, ... capped at 4s
        let ms = 100u64 << self.poll_count.min(5);
        Duration::from_millis(ms)
    }
}

impl R2GlassApp {
    pub(crate) fn start_background_job(&mut self, label: &str, command: &str, refresh_after: bool) {
        self.start_background_job_with_fallback(label, command, refresh_after, None);
    }

    pub(crate) fn start_background_job_with_fallback(
        &mut self,
        label: &str,
        command: &str,
        refresh_after: bool,
        fallback_command: Option<String>,
    ) {
        let Some(session) = self.session.as_mut() else {
            "No target loaded".clone_into(&mut self.status);
            return;
        };
        if self.background_job.is_some() {
            "Background command already running".clone_into(&mut self.status);
            return;
        }
        match session.command_receiver(command) {
            Ok(receiver) => {
                self.status = format!("Running {label}...");
                self.background_job = Some(BackgroundJob::new(
                    label,
                    command,
                    refresh_after,
                    fallback_command,
                    receiver,
                ));
            }
            Err(err) => self.status = err.to_string(),
        }
    }

    pub(crate) fn poll_background_job(&mut self, ctx: &egui::Context) {
        let Some(job) = self.background_job.as_mut() else {
            return;
        };
        match job.receiver.try_recv() {
            Ok(result) => self.finish_background_job(result),
            Err(TryRecvError::Empty) => {
                self.status = format!("Running {}...", job.label);
                ctx.request_repaint_after(job.poll_interval());
                job.poll_count += 1;
            }
            Err(TryRecvError::Disconnected) => {
                let label = job.label.clone();
                self.background_job = None;
                self.output = format!(
                    "Background job '{label}' disconnected — the r2 worker may have crashed. Try reloading the target."
                );
                self.status = "Background command disconnected".to_owned();
            }
        }
    }

    fn finish_background_job(&mut self, result: JobResult) {
        let Some(job) = self.background_job.take() else {
            return;
        };
        match result {
            Ok(output) if decompiler_crashed(&output) => {
                if let Some(fallback) = job.fallback_command {
                    self.start_background_job("pseudo-code fallback", &fallback, job.refresh_after);
                    self.status =
                        "r2dec crashed; running radare2 pseudo-code fallback.".to_owned();
                    // Don't overwrite output — fallback will replace it when done
                } else {
                    self.output = output;
                    self.status = format!("{} failed", job.label);
                }
            }
            Ok(output) => {
                self.output = output;
                self.history.push(HistoryEntry::command(job.command));
                self.status = format!("Finished {}", job.label);
                if job.refresh_after {
                    self.refresh_view();
                }
            }
            Err(err) => {
                self.output = format!("{} failed: {err}", job.label);
                self.status = format!("{} failed", job.label);
            }
        }
    }
}
