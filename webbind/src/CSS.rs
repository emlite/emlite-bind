use super::*;

pub fn escape(ident: jsbind::CSSOMString) -> jsbind::CSSOMString {
    emlite::Val::global("CSS")
        .call("escape", &[ident.into()])
        .as_::<jsbind::CSSOMString>()
}
