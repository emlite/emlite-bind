use super::*;


pub fn escape(ident: CSSOMString) -> CSSOMString {
    emlite::Val::global("CSS").call("escape", &[ident.into(), ]).as_::<CSSOMString>()
}

