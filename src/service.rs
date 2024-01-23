#![allow(unused)]
use crate::storage::{self, add_item as storage_add_item, update_item};

pub fn add_item() {
    println!("service::add_item");
    storage_add_item();
}

pub fn complete_item() {
    println!("service::complete_item");
    update_item();
}

pub fn uncomplete_item() {
    println!("service::uncomplete_item");
    update_item();
}

pub fn shelve_item() {
    println!("service::shelve_item");
    update_item();
}

pub fn delete_item() {
    println!("service::delete_item");
    update_item();
}

pub fn restore_item() {
    println!("service::restore_item");
    update_item();
}

pub fn destroy_item() {
    println!("service::destroy_item");
    storage::delete_item();
}

pub fn destory_all_items() {
    println!("service::destory_all_items");
    storage::delete_item();
}

pub fn list_uncompleted_items() {
    println!("service::list_uncompleted_items");
    storage::get_all();
}

pub fn list_completed_items() {
    println!("service::list_completed_items");
    storage::get_all();
}

pub fn list_shelved_items() {
    println!("service::list_shelved_items");
    storage::get_all();
}

pub fn list_all_items() {
    println!("service::list_all_items");
    storage::get_all();
}

pub fn list_deleted_items() {
    println!("service::list_deleted_items");
    storage::get_all();
}