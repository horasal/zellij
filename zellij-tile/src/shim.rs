use serde::{de::DeserializeOwned, Serialize};
use std::{io, path::Path};

use crate::data::*;

// Subscription Handling

pub fn subscribe(event_types: &[EventType]) {
    object_to_stdout(&event_types);
    unsafe { host_subscribe() };
}

pub fn unsubscribe(event_types: &[EventType]) {
    object_to_stdout(&event_types);
    unsafe { host_unsubscribe() };
}

// Plugin Settings

pub fn set_max_height(max_height: i32) {
    unsafe { host_set_max_height(max_height) };
}

pub fn set_selectable(selectable: bool) {
    unsafe { host_set_selectable(if selectable { 1 } else { 0 }) };
}

pub fn set_invisible_borders(invisible_borders: bool) {
    unsafe { host_set_invisible_borders(if invisible_borders { 1 } else { 0 }) };
}

// Query Functions
pub fn get_plugin_ids() -> PluginIds {
    unsafe { host_get_plugin_ids() };
    object_from_stdin()
}

// Host Functions

pub fn open_file(path: &Path) {
    object_to_stdout(&path);
    unsafe { host_open_file() };
}

pub fn set_timeout(secs: f64) {
    unsafe { host_set_timeout(secs) };
}

// Internal Functions

#[doc(hidden)]
pub fn object_from_stdin<T: DeserializeOwned>() -> T {
    let mut json = String::new();
    io::stdin().read_line(&mut json).unwrap();
    serde_json::from_str(&json).unwrap()
}

#[doc(hidden)]
pub fn object_to_stdout(object: &impl Serialize) {
    println!("{}", serde_json::to_string(object).unwrap());
}

#[link(wasm_import_module = "zellij")]
extern "C" {
    fn host_subscribe();
    fn host_unsubscribe();
    fn host_set_max_height(max_height: i32);
    fn host_set_selectable(selectable: i32);
    fn host_set_invisible_borders(invisible_borders: i32);
    fn host_get_plugin_ids();
    fn host_open_file();
    fn host_set_timeout(secs: f64);
}
