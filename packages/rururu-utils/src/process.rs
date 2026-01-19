use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::process::{Child, Command, Stdio};
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Failed to spawn process: {0}")]
    SpawnError(String),
    #[error("Failed to send signal: {0}")]
    SignalError(String),
    #[error("Process not found: {0}")]
    NotFound(i32),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessPriority {
    RealTime,
    High,
    Normal,
    Low,
    Idle,
}

impl ProcessPriority {
    pub fn nice_value(&self) -> i32 {
        match self {
            ProcessPriority::RealTime => -20,
            ProcessPriority::High => -10,
            ProcessPriority::Normal => 0,
            ProcessPriority::Low => 10,
            ProcessPriority::Idle => 19,
        }
    }
}

pub struct ManagedProcess {
    child: Child,
    name: String,
}

impl ManagedProcess {
    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_running(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(_)) => false,
            Ok(None) => true,
            Err(_) => false,
        }
    }

    pub fn wait(&mut self) -> Result<i32, ProcessError> {
        let status = self.child.wait()?;
        Ok(status.code().unwrap_or(-1))
    }

    pub fn kill(&mut self) -> Result<(), ProcessError> {
        self.child.kill()?;
        Ok(())
    }

    pub fn terminate(&self) -> Result<(), ProcessError> {
        signal::kill(Pid::from_raw(self.child.id() as i32), Signal::SIGTERM)
            .map_err(|e| ProcessError::SignalError(e.to_string()))
    }
}

pub struct ProcessManager {
    managed: Vec<ManagedProcess>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            managed: Vec::new(),
        }
    }

    pub fn spawn(&mut self, name: &str, program: &str, args: &[&str]) -> Result<u32, ProcessError> {
        info!("Spawning process: {} {}", program, args.join(" "));

        let child = Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ProcessError::SpawnError(e.to_string()))?;

        let pid = child.id();
        let managed = ManagedProcess {
            child,
            name: name.to_string(),
        };

        self.managed.push(managed);
        debug!("Process {} started with PID {}", name, pid);

        Ok(pid)
    }

    pub fn spawn_daemon(
        &mut self,
        name: &str,
        program: &str,
        args: &[&str],
    ) -> Result<u32, ProcessError> {
        info!("Spawning daemon: {} {}", program, args.join(" "));

        let child = Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| ProcessError::SpawnError(e.to_string()))?;

        let pid = child.id();
        let managed = ManagedProcess {
            child,
            name: name.to_string(),
        };

        self.managed.push(managed);
        debug!("Daemon {} started with PID {}", name, pid);

        Ok(pid)
    }

    pub fn kill_by_name(&mut self, name: &str) -> Result<usize, ProcessError> {
        let mut killed = 0;

        for proc in &mut self.managed {
            if proc.name == name && proc.is_running() {
                match proc.kill() {
                    Ok(_) => killed += 1,
                    Err(e) => warn!("Failed to kill {}: {}", name, e),
                }
            }
        }

        Ok(killed)
    }

    pub fn terminate_by_name(&self, name: &str) -> Result<usize, ProcessError> {
        let mut terminated = 0;

        for proc in &self.managed {
            if proc.name == name {
                match proc.terminate() {
                    Ok(_) => terminated += 1,
                    Err(e) => warn!("Failed to terminate {}: {}", name, e),
                }
            }
        }

        Ok(terminated)
    }

    pub fn kill_pid(pid: i32) -> Result<(), ProcessError> {
        signal::kill(Pid::from_raw(pid), Signal::SIGKILL)
            .map_err(|e| ProcessError::SignalError(e.to_string()))
    }

    pub fn terminate_pid(pid: i32) -> Result<(), ProcessError> {
        signal::kill(Pid::from_raw(pid), Signal::SIGTERM)
            .map_err(|e| ProcessError::SignalError(e.to_string()))
    }

    pub fn send_signal(pid: i32, sig: Signal) -> Result<(), ProcessError> {
        signal::kill(Pid::from_raw(pid), sig).map_err(|e| ProcessError::SignalError(e.to_string()))
    }

    pub fn set_priority(pid: i32, priority: ProcessPriority) -> Result<(), ProcessError> {
        let nice = priority.nice_value();

        // Use renice via command for simplicity
        let output = Command::new("renice")
            .args([&nice.to_string(), "-p", &pid.to_string()])
            .output()?;

        if !output.status.success() {
            return Err(ProcessError::SignalError(format!(
                "Failed to set priority: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    pub fn list_managed(&self) -> Vec<(&str, u32)> {
        self.managed
            .iter()
            .map(|p| (p.name.as_str(), p.child.id()))
            .collect()
    }

    pub fn cleanup(&mut self) {
        self.managed.retain_mut(|p| p.is_running());
    }

    pub fn shutdown_all(&mut self) {
        info!("Shutting down all managed processes");

        // First try graceful termination
        for proc in &self.managed {
            let _ = proc.terminate();
        }

        // Wait a bit then force kill remaining
        std::thread::sleep(std::time::Duration::from_secs(2));

        for proc in &mut self.managed {
            if proc.is_running() {
                let _ = proc.kill();
            }
        }

        self.managed.clear();
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ProcessManager {
    fn drop(&mut self) {
        // Clean up managed processes on drop
        for proc in &mut self.managed {
            if proc.is_running() {
                let _ = proc.kill();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_priority() {
        assert_eq!(ProcessPriority::RealTime.nice_value(), -20);
        assert_eq!(ProcessPriority::Normal.nice_value(), 0);
        assert_eq!(ProcessPriority::Idle.nice_value(), 19);
    }

    #[test]
    fn test_spawn_process() {
        let mut pm = ProcessManager::new();
        let result = pm.spawn("test", "echo", &["hello"]);
        assert!(result.is_ok());
    }
}
