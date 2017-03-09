#![crate_name = "rocket_swagger"]
#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

mod macros;

use rustc_plugin::Registry;

pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("rocket_swagger", macros::rocket_swagger);
}