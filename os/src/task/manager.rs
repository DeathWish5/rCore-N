use super::{current_task, TaskControlBlock};
use alloc::collections::VecDeque;
use alloc::sync::Arc;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn remove(&mut self, task: &Arc<TaskControlBlock>) {
        for (idx, task_item) in self.ready_queue.iter().enumerate() {
            if *task_item == *task {
                self.ready_queue.remove(idx);
                break;
            }
        }
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // May need to concern affinity
        self.ready_queue.pop_front()
    }
}

// lazy_static! {
//     pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
// }

// pub fn add_task(task: Arc<TaskControlBlock>) {
//     TASK_MANAGER.lock().add(task);
// }

// pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
//     TASK_MANAGER.lock().fetch()
// }

// pub fn find_task(pid: usize) -> Option<Arc<TaskControlBlock>> {
//     let current = current_task().unwrap();
//     if current.pid == pid {
//         return Some(current);
//     }
//     TASK_MANAGER.lock().find(pid)
// }
