use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use r2pipe::r2pipe::R2Pipe;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum R2Error {
    #[error("target path does not exist: {0}")]
    MissingTarget(PathBuf),
    #[error("radare2 command failed: {0}")]
    Command(String),
}

pub struct R2Session {
    sender: Sender<WorkerCommand>,
    worker: Option<JoinHandle<()>>,
    dead: bool,
}

enum WorkerCommand {
    Run {
        command: String,
        response: Sender<Result<String, R2Error>>,
    },
    Close,
}

impl R2Session {
    /// Starts a persistent r2 pipe for an existing target file.
    ///
    /// # Errors
    ///
    /// Returns an error when the target path is missing or radare2 cannot be
    /// spawned/configured.
    pub fn open(target: &Path) -> Result<Self, R2Error> {
        if !target.exists() {
            return Err(R2Error::MissingTarget(target.to_path_buf()));
        }

        let (sender, receiver) = mpsc::channel();
        let (ready_sender, ready_receiver) = mpsc::channel();
        let target = target.to_path_buf();
        let worker = thread::spawn(move || worker_main(&target, &receiver, &ready_sender));

        match ready_receiver.recv() {
            Ok(Ok(())) => Ok(Self {
                sender,
                worker: Some(worker),
                dead: false,
            }),
            Ok(Err(err)) => Err(err),
            Err(err) => Err(R2Error::Command(err.to_string())),
        }
    }

    /// Runs one radare2 command in the current session.
    ///
    /// # Errors
    ///
    /// Returns an error when the underlying r2 pipe rejects the command or the
    /// process can no longer be reached.
    pub fn command(&mut self, command: &str) -> Result<String, R2Error> {
        if self.dead {
            return Err(R2Error::Command("r2 session is no longer running".to_owned()));
        }
        let recv_result = self.command_receiver(command)?.recv();
        recv_result
            .map_err(|err| {
                self.dead = true;
                R2Error::Command(err.to_string())
            })?
    }

    /// Queues one radare2 command and returns a receiver for its eventual output.
    ///
    /// # Errors
    ///
    /// Returns an error when the r2 worker thread is no longer reachable.
    pub fn command_receiver(
        &mut self,
        command: &str,
    ) -> Result<Receiver<Result<String, R2Error>>, R2Error> {
        if self.dead {
            return Err(R2Error::Command("r2 session is no longer running".to_owned()));
        }
        let (response, receiver) = mpsc::channel();
        self.sender
            .send(WorkerCommand::Run {
                command: command.to_owned(),
                response,
            })
            .map_err(|err| {
                self.dead = true;
                R2Error::Command(err.to_string())
            })?;
        Ok(receiver)
    }
}

impl Drop for R2Session {
    fn drop(&mut self) {
        let _ignored = self.sender.send(WorkerCommand::Close);
        if let Some(worker) = self.worker.take() {
            let _ignored = worker.join();
        }
    }
}

fn worker_main(
    target: &Path,
    receiver: &Receiver<WorkerCommand>,
    ready: &Sender<Result<(), R2Error>>,
) {
    let setup = open_pipe(target);
    let Ok(mut pipe) = setup else {
        let err = setup.err().map_or_else(
            || R2Error::Command("failed to open r2".to_owned()),
            |err| R2Error::Command(err.to_string()),
        );
        let _ignored = ready.send(Err(err));
        return;
    };
    if let Err(err) = configure_pipe(&mut pipe) {
        let _ignored = ready.send(Err(err));
        return;
    }
    let _ignored = ready.send(Ok(()));
    worker_loop(&mut pipe, receiver);
}

fn open_pipe(target: &Path) -> Result<R2Pipe, r2pipe::Error> {
    R2Pipe::spawn(target.to_string_lossy(), None)
}

fn configure_pipe(pipe: &mut R2Pipe) -> Result<(), R2Error> {
    for command in ["e scr.color=false", "e io.cache=true"] {
        pipe.cmd(command)
            .map_err(|err| R2Error::Command(err.to_string()))?;
    }
    Ok(())
}

fn worker_loop(pipe: &mut R2Pipe, receiver: &Receiver<WorkerCommand>) {
    while let Ok(message) = receiver.recv() {
        match message {
            WorkerCommand::Run { command, response } => {
                let result = pipe
                    .cmd(&command)
                    .map_err(|err| R2Error::Command(err.to_string()));
                let _ignored = response.send(result);
            }
            WorkerCommand::Close => {
                pipe.close();
                break;
            }
        }
    }
}
