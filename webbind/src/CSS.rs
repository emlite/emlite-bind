use super::*;

pub fn escape(ident: &JsString) -> JsString {
    Any::global("CSS")
        .call("escape", &[ident.into()])
        .as_::<JsString>()
}
