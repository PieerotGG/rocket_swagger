#![crate_name = "rocket_swagger"]
#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::ext::quote::rt::Span;

pub fn rocket_swagger(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let mut parser = ecx.new_parser_from_tts(args);

    println!("{:?}", parser.parse_item());
    DummyResult::any(sp)
}


pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("rocket_swagger", rocket_swagger);
}