use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::ext::quote::rt::Span;

#[rustfmt_skip]
pub fn rocket_swagger(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let mut parser = ecx.new_parser_from_tts(args);

    println!("{:?}", parser.parse_item());
    DummyResult::any(sp)
}
