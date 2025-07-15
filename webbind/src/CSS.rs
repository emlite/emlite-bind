use super::*;

pub fn escape(ident: &str) -> String {
    emlite::Val::global("CSS")
        .call("escape", &[ident.into()])
        .as_::<String>()
}
