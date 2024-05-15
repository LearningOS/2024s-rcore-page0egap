//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;
use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task statistics
    pub statitics: Statistics,
}

#[derive(Copy, Clone)]
pub struct Statistics {
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    // start time(us)
    pub time: usize,
    pub is_first: bool,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
            is_first: true,
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
