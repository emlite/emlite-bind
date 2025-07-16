use super::*;

pub fn escape(ident: &str) -> String {
    Any::global("CSS")
        .call("escape", &[ident.into()])
        .as_::<String>()
}
