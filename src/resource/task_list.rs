use crate::model::Task;

#[derive(Clone, Default, Debug)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, index: usize) {
        self.tasks.remove(index);
    }
}
