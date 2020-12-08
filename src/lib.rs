
#![no_std]

imports!();

#[elrond_wasm_derive::contract(TodoImpl)]
pub trait Todo {
    #[init]
    fn init(&self) {}

    #[endpoint]
    #[storage_set("todo")]
    fn set_todo(&self, id: u16, content: &Vec<u8>);

    #[view(getTodos)]
    #[storage_get("todo")]
    fn get_todo(&self, id: u16) -> Vec<u8>;
        

}
