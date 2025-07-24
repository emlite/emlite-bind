use super::*;

pub fn escape(ident: &CSSOMString) -> CSSOMString {
    Any::global("CSS")
        .call("escape", &[ident.into()])
        .as_::<CSSOMString>()
}
