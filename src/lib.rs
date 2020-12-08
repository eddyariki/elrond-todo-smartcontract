
#![no_std]

imports!();

mod task;

use task::Task;


#[elrond_wasm_derive::contract(TodoImpl)]
pub trait Todo {
    #[init]
    fn init(&self) {}

    #[endpoint(addTask)]
    fn add_task(&self, id: u16, content: Vec<u8>)->SCResult<()>{

        require!(!content.is_empty(), "Content cannot be empty!");

        let task = Task{
            content,
            completed:false
        };
    
        self.set_task(&id, &task);
    
        Ok(())
    }

    #[view(getTaskContent)]
    fn get_content(&self, id: &u16) -> Vec<u8> {
        let task: Task = self.get_task(&id);
        return task.content;
    }
    #[view(getTaskStatus)]
    fn get_task_status(&self, id:&u16) -> bool{
        let task: Task = self.get_task(&id);
        return task.completed;
    }

    #[endpoint(checkTask)]
    fn check_task(&self, id: u16) -> SCResult<()>{
        let mut task = self.get_mut_task(&id);
        task.completed = !task.completed;
        Ok(())
    }

    #[storage_set("task")]
    fn set_task(&self, id: &u16, task: &Task); 

    #[view(getMutTasks)]
    #[storage_get_mut("task")]
    fn get_mut_task(&self, id: &u16) -> mut_storage!(Task);

    #[view(getTask)]
    #[storage_get("task")]
    fn get_task(&self, id: &u16)-> Task;

}


