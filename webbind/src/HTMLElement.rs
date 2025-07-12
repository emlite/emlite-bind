use super::*;

#[derive(Clone, Debug)]
pub struct ShowPopoverOptions {
    inner: emlite::Val,
}
impl FromVal for ShowPopoverOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ShowPopoverOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ShowPopoverOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ShowPopoverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ShowPopoverOptions> for emlite::Val {
    fn from(s: ShowPopoverOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ShowPopoverOptions {
    pub fn source(&self) -> HTMLElement {
        self.inner.get("source").as_::<HTMLElement>()
    }

    pub fn set_source(&mut self, value: HTMLElement) {
        self.inner.set("source", value);
    }
}
#[derive(Clone, Debug)]
pub struct HTMLElement {
    inner: Element,
}
impl FromVal for HTMLElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLElement {
            inner: Element::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HTMLElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLElement> for emlite::Val {
    fn from(s: HTMLElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLElement {
    pub fn new() -> HTMLElement {
        Self {
            inner: emlite::Val::global("HTMLElement").new(&[]).as_::<Element>(),
        }
    }
}
impl HTMLElement {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }

    pub fn set_title(&mut self, value: jsbind::DOMString) {
        self.inner.set("title", value);
    }
}
impl HTMLElement {
    pub fn lang(&self) -> jsbind::DOMString {
        self.inner.get("lang").as_::<jsbind::DOMString>()
    }

    pub fn set_lang(&mut self, value: jsbind::DOMString) {
        self.inner.set("lang", value);
    }
}
impl HTMLElement {
    pub fn translate(&self) -> bool {
        self.inner.get("translate").as_::<bool>()
    }

    pub fn set_translate(&mut self, value: bool) {
        self.inner.set("translate", value);
    }
}
impl HTMLElement {
    pub fn dir(&self) -> jsbind::DOMString {
        self.inner.get("dir").as_::<jsbind::DOMString>()
    }

    pub fn set_dir(&mut self, value: jsbind::DOMString) {
        self.inner.set("dir", value);
    }
}
impl HTMLElement {
    pub fn hidden(&self) -> jsbind::Any {
        self.inner.get("hidden").as_::<jsbind::Any>()
    }

    pub fn set_hidden(&mut self, value: jsbind::Any) {
        self.inner.set("hidden", value);
    }
}
impl HTMLElement {
    pub fn inert(&self) -> bool {
        self.inner.get("inert").as_::<bool>()
    }

    pub fn set_inert(&mut self, value: bool) {
        self.inner.set("inert", value);
    }
}
impl HTMLElement {
    pub fn click(&self) -> jsbind::Undefined {
        self.inner.call("click", &[]).as_::<jsbind::Undefined>()
    }
}
impl HTMLElement {
    pub fn access_key(&self) -> jsbind::DOMString {
        self.inner.get("accessKey").as_::<jsbind::DOMString>()
    }

    pub fn set_access_key(&mut self, value: jsbind::DOMString) {
        self.inner.set("accessKey", value);
    }
}
impl HTMLElement {
    pub fn access_key_label(&self) -> jsbind::DOMString {
        self.inner.get("accessKeyLabel").as_::<jsbind::DOMString>()
    }
}
impl HTMLElement {
    pub fn draggable(&self) -> bool {
        self.inner.get("draggable").as_::<bool>()
    }

    pub fn set_draggable(&mut self, value: bool) {
        self.inner.set("draggable", value);
    }
}
impl HTMLElement {
    pub fn spellcheck(&self) -> bool {
        self.inner.get("spellcheck").as_::<bool>()
    }

    pub fn set_spellcheck(&mut self, value: bool) {
        self.inner.set("spellcheck", value);
    }
}
impl HTMLElement {
    pub fn writing_suggestions(&self) -> jsbind::DOMString {
        self.inner
            .get("writingSuggestions")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_writing_suggestions(&mut self, value: jsbind::DOMString) {
        self.inner.set("writingSuggestions", value);
    }
}
impl HTMLElement {
    pub fn autocapitalize(&self) -> jsbind::DOMString {
        self.inner.get("autocapitalize").as_::<jsbind::DOMString>()
    }

    pub fn set_autocapitalize(&mut self, value: jsbind::DOMString) {
        self.inner.set("autocapitalize", value);
    }
}
impl HTMLElement {
    pub fn autocorrect(&self) -> bool {
        self.inner.get("autocorrect").as_::<bool>()
    }

    pub fn set_autocorrect(&mut self, value: bool) {
        self.inner.set("autocorrect", value);
    }
}
impl HTMLElement {
    pub fn inner_text(&self) -> jsbind::DOMString {
        self.inner.get("innerText").as_::<jsbind::DOMString>()
    }

    pub fn set_inner_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("innerText", value);
    }
}
impl HTMLElement {
    pub fn outer_text(&self) -> jsbind::DOMString {
        self.inner.get("outerText").as_::<jsbind::DOMString>()
    }

    pub fn set_outer_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("outerText", value);
    }
}
impl HTMLElement {
    pub fn attach_internals(&self) -> ElementInternals {
        self.inner
            .call("attachInternals", &[])
            .as_::<ElementInternals>()
    }
}
impl HTMLElement {
    pub fn show_popover0(&self) -> jsbind::Undefined {
        self.inner
            .call("showPopover", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn show_popover1(&self, options: ShowPopoverOptions) -> jsbind::Undefined {
        self.inner
            .call("showPopover", &[options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLElement {
    pub fn hide_popover(&self) -> jsbind::Undefined {
        self.inner
            .call("hidePopover", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLElement {
    pub fn toggle_popover0(&self) -> bool {
        self.inner.call("togglePopover", &[]).as_::<bool>()
    }

    pub fn toggle_popover1(&self, options: jsbind::Any) -> bool {
        self.inner
            .call("togglePopover", &[options.into()])
            .as_::<bool>()
    }
}
impl HTMLElement {
    pub fn popover(&self) -> jsbind::DOMString {
        self.inner.get("popover").as_::<jsbind::DOMString>()
    }

    pub fn set_popover(&mut self, value: jsbind::DOMString) {
        self.inner.set("popover", value);
    }
}
impl HTMLElement {
    pub fn scroll_parent(&self) -> Element {
        self.inner.get("scrollParent").as_::<Element>()
    }
}
impl HTMLElement {
    pub fn offset_parent(&self) -> Element {
        self.inner.get("offsetParent").as_::<Element>()
    }
}
impl HTMLElement {
    pub fn offset_top(&self) -> i32 {
        self.inner.get("offsetTop").as_::<i32>()
    }
}
impl HTMLElement {
    pub fn offset_left(&self) -> i32 {
        self.inner.get("offsetLeft").as_::<i32>()
    }
}
impl HTMLElement {
    pub fn offset_width(&self) -> i32 {
        self.inner.get("offsetWidth").as_::<i32>()
    }
}
impl HTMLElement {
    pub fn offset_height(&self) -> i32 {
        self.inner.get("offsetHeight").as_::<i32>()
    }
}
impl HTMLElement {
    pub fn edit_context(&self) -> EditContext {
        self.inner.get("editContext").as_::<EditContext>()
    }

    pub fn set_edit_context(&mut self, value: EditContext) {
        self.inner.set("editContext", value);
    }
}
impl HTMLElement {
    pub fn style(&self) -> CSSStyleDeclaration {
        self.inner.get("style").as_::<CSSStyleDeclaration>()
    }
}
impl HTMLElement {
    pub fn onbeforexrselect(&self) -> jsbind::Any {
        self.inner.get("onbeforexrselect").as_::<jsbind::Any>()
    }

    pub fn set_onbeforexrselect(&mut self, value: jsbind::Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
impl HTMLElement {
    pub fn virtual_keyboard_policy(&self) -> jsbind::DOMString {
        self.inner
            .get("virtualKeyboardPolicy")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_virtual_keyboard_policy(&mut self, value: jsbind::DOMString) {
        self.inner.set("virtualKeyboardPolicy", value);
    }
}
impl HTMLElement {
    pub fn dataset(&self) -> DOMStringMap {
        self.inner.get("dataset").as_::<DOMStringMap>()
    }
}
impl HTMLElement {
    pub fn nonce(&self) -> jsbind::DOMString {
        self.inner.get("nonce").as_::<jsbind::DOMString>()
    }

    pub fn set_nonce(&mut self, value: jsbind::DOMString) {
        self.inner.set("nonce", value);
    }
}
impl HTMLElement {
    pub fn autofocus(&self) -> bool {
        self.inner.get("autofocus").as_::<bool>()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.inner.set("autofocus", value);
    }
}
impl HTMLElement {
    pub fn tab_index(&self) -> i32 {
        self.inner.get("tabIndex").as_::<i32>()
    }

    pub fn set_tab_index(&mut self, value: i32) {
        self.inner.set("tabIndex", value);
    }
}
impl HTMLElement {
    pub fn focus0(&self) -> jsbind::Undefined {
        self.inner.call("focus", &[]).as_::<jsbind::Undefined>()
    }

    pub fn focus1(&self, options: FocusOptions) -> jsbind::Undefined {
        self.inner
            .call("focus", &[options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLElement {
    pub fn blur(&self) -> jsbind::Undefined {
        self.inner.call("blur", &[]).as_::<jsbind::Undefined>()
    }
}
