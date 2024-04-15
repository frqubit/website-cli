use super::{Parse, ParseError};
use super::Expr;


pub enum Item {
    Let(ItemLet)
}

impl Item {
    pub fn new_let(ident: String, expr: Expr) -> Self {
        Item::Let(ItemLet { ident, expr })
    }
}

impl Parse for Item {
    fn parse(s: &str) -> Result<Self, ParseError> {
        crate::parse::ItemParser::new().parse(s)
    }
}

pub struct ItemLet {
    ident: String,
    expr: Expr
}
