use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShowPopoverOptions {
    inner: Any,
}
impl FromVal for ShowPopoverOptions {
    fn from_val(v: &Any) -> Self {
        ShowPopoverOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ShowPopoverOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ShowPopoverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ShowPopoverOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ShowPopoverOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ShowPopoverOptions> for Any {
    fn from(s: ShowPopoverOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ShowPopoverOptions> for Any {
    fn from(s: &ShowPopoverOptions) -> Any {
        s.inner.clone()
    }
}

impl ShowPopoverOptions {
    pub fn source(&self) -> HTMLElement {
        self.inner.get("source").as_::<HTMLElement>()
    }

    pub fn set_source(&mut self, value: &HTMLElement) {
        self.inner.set("source", value);
    }
}
/// The HTMLElement class.
/// [`HTMLElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLElement {
    inner: Element,
}
impl FromVal for HTMLElement {
    fn from_val(v: &Any) -> Self {
        HTMLElement {
            inner: Element::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLElement> for Any {
    fn from(s: HTMLElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLElement> for Any {
    fn from(s: &HTMLElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLElement);

impl HTMLElement {
    /// The `new HTMLElement(..)` constructor, creating a new HTMLElement instance
    pub fn new() -> HTMLElement {
        Self {
            inner: Any::global("HTMLElement").new(&[]).as_::<Element>(),
        }
    }
}
impl HTMLElement {
    /// Getter of the `title` attribute.
    /// [`HTMLElement.title`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title)
    pub fn title(&self) -> String {
        self.inner.get("title").as_::<String>()
    }

    /// Setter of the `title` attribute.
    /// [`HTMLElement.title`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title)
    pub fn set_title(&mut self, value: &str) {
        self.inner.set("title", value);
    }
}
impl HTMLElement {
    /// Getter of the `lang` attribute.
    /// [`HTMLElement.lang`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang)
    pub fn lang(&self) -> String {
        self.inner.get("lang").as_::<String>()
    }

    /// Setter of the `lang` attribute.
    /// [`HTMLElement.lang`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang)
    pub fn set_lang(&mut self, value: &str) {
        self.inner.set("lang", value);
    }
}
impl HTMLElement {
    /// Getter of the `translate` attribute.
    /// [`HTMLElement.translate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/translate)
    pub fn translate(&self) -> bool {
        self.inner.get("translate").as_::<bool>()
    }

    /// Setter of the `translate` attribute.
    /// [`HTMLElement.translate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/translate)
    pub fn set_translate(&mut self, value: bool) {
        self.inner.set("translate", value);
    }
}
impl HTMLElement {
    /// Getter of the `dir` attribute.
    /// [`HTMLElement.dir`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir)
    pub fn dir(&self) -> String {
        self.inner.get("dir").as_::<String>()
    }

    /// Setter of the `dir` attribute.
    /// [`HTMLElement.dir`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir)
    pub fn set_dir(&mut self, value: &str) {
        self.inner.set("dir", value);
    }
}
impl HTMLElement {
    /// Getter of the `hidden` attribute.
    /// [`HTMLElement.hidden`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden)
    pub fn hidden(&self) -> Any {
        self.inner.get("hidden").as_::<Any>()
    }

    /// Setter of the `hidden` attribute.
    /// [`HTMLElement.hidden`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden)
    pub fn set_hidden(&mut self, value: &Any) {
        self.inner.set("hidden", value);
    }
}
impl HTMLElement {
    /// Getter of the `inert` attribute.
    /// [`HTMLElement.inert`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/inert)
    pub fn inert(&self) -> bool {
        self.inner.get("inert").as_::<bool>()
    }

    /// Setter of the `inert` attribute.
    /// [`HTMLElement.inert`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/inert)
    pub fn set_inert(&mut self, value: bool) {
        self.inner.set("inert", value);
    }
}
impl HTMLElement {
    /// The click method.
    /// [`HTMLElement.click`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click)
    pub fn click(&self) -> Undefined {
        self.inner.call("click", &[]).as_::<Undefined>()
    }
}
impl HTMLElement {
    /// Getter of the `accessKey` attribute.
    /// [`HTMLElement.accessKey`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)
    pub fn access_key(&self) -> String {
        self.inner.get("accessKey").as_::<String>()
    }

    /// Setter of the `accessKey` attribute.
    /// [`HTMLElement.accessKey`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)
    pub fn set_access_key(&mut self, value: &str) {
        self.inner.set("accessKey", value);
    }
}
impl HTMLElement {
    /// Getter of the `accessKeyLabel` attribute.
    /// [`HTMLElement.accessKeyLabel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKeyLabel)
    pub fn access_key_label(&self) -> String {
        self.inner.get("accessKeyLabel").as_::<String>()
    }
}
impl HTMLElement {
    /// Getter of the `draggable` attribute.
    /// [`HTMLElement.draggable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable)
    pub fn draggable(&self) -> bool {
        self.inner.get("draggable").as_::<bool>()
    }

    /// Setter of the `draggable` attribute.
    /// [`HTMLElement.draggable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable)
    pub fn set_draggable(&mut self, value: bool) {
        self.inner.set("draggable", value);
    }
}
impl HTMLElement {
    /// Getter of the `spellcheck` attribute.
    /// [`HTMLElement.spellcheck`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck)
    pub fn spellcheck(&self) -> bool {
        self.inner.get("spellcheck").as_::<bool>()
    }

    /// Setter of the `spellcheck` attribute.
    /// [`HTMLElement.spellcheck`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck)
    pub fn set_spellcheck(&mut self, value: bool) {
        self.inner.set("spellcheck", value);
    }
}
impl HTMLElement {
    /// Getter of the `writingSuggestions` attribute.
    /// [`HTMLElement.writingSuggestions`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/writingSuggestions)
    pub fn writing_suggestions(&self) -> String {
        self.inner.get("writingSuggestions").as_::<String>()
    }

    /// Setter of the `writingSuggestions` attribute.
    /// [`HTMLElement.writingSuggestions`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/writingSuggestions)
    pub fn set_writing_suggestions(&mut self, value: &str) {
        self.inner.set("writingSuggestions", value);
    }
}
impl HTMLElement {
    /// Getter of the `autocapitalize` attribute.
    /// [`HTMLElement.autocapitalize`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autocapitalize)
    pub fn autocapitalize(&self) -> String {
        self.inner.get("autocapitalize").as_::<String>()
    }

    /// Setter of the `autocapitalize` attribute.
    /// [`HTMLElement.autocapitalize`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autocapitalize)
    pub fn set_autocapitalize(&mut self, value: &str) {
        self.inner.set("autocapitalize", value);
    }
}
impl HTMLElement {
    /// Getter of the `autocorrect` attribute.
    /// [`HTMLElement.autocorrect`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autocorrect)
    pub fn autocorrect(&self) -> bool {
        self.inner.get("autocorrect").as_::<bool>()
    }

    /// Setter of the `autocorrect` attribute.
    /// [`HTMLElement.autocorrect`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autocorrect)
    pub fn set_autocorrect(&mut self, value: bool) {
        self.inner.set("autocorrect", value);
    }
}
impl HTMLElement {
    /// Getter of the `innerText` attribute.
    /// [`HTMLElement.innerText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)
    pub fn inner_text(&self) -> String {
        self.inner.get("innerText").as_::<String>()
    }

    /// Setter of the `innerText` attribute.
    /// [`HTMLElement.innerText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)
    pub fn set_inner_text(&mut self, value: &str) {
        self.inner.set("innerText", value);
    }
}
impl HTMLElement {
    /// Getter of the `outerText` attribute.
    /// [`HTMLElement.outerText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/outerText)
    pub fn outer_text(&self) -> String {
        self.inner.get("outerText").as_::<String>()
    }

    /// Setter of the `outerText` attribute.
    /// [`HTMLElement.outerText`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/outerText)
    pub fn set_outer_text(&mut self, value: &str) {
        self.inner.set("outerText", value);
    }
}
impl HTMLElement {
    /// The attachInternals method.
    /// [`HTMLElement.attachInternals`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/attachInternals)
    pub fn attach_internals(&self) -> ElementInternals {
        self.inner
            .call("attachInternals", &[])
            .as_::<ElementInternals>()
    }
}
impl HTMLElement {
    /// The showPopover method.
    /// [`HTMLElement.showPopover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/showPopover)
    pub fn show_popover0(&self) -> Undefined {
        self.inner.call("showPopover", &[]).as_::<Undefined>()
    }
    /// The showPopover method.
    /// [`HTMLElement.showPopover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/showPopover)
    pub fn show_popover1(&self, options: &ShowPopoverOptions) -> Undefined {
        self.inner
            .call("showPopover", &[options.into()])
            .as_::<Undefined>()
    }
}
impl HTMLElement {
    /// The hidePopover method.
    /// [`HTMLElement.hidePopover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidePopover)
    pub fn hide_popover(&self) -> Undefined {
        self.inner.call("hidePopover", &[]).as_::<Undefined>()
    }
}
impl HTMLElement {
    /// The togglePopover method.
    /// [`HTMLElement.togglePopover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/togglePopover)
    pub fn toggle_popover0(&self) -> bool {
        self.inner.call("togglePopover", &[]).as_::<bool>()
    }
    /// The togglePopover method.
    /// [`HTMLElement.togglePopover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/togglePopover)
    pub fn toggle_popover1(&self, options: &Any) -> bool {
        self.inner
            .call("togglePopover", &[options.into()])
            .as_::<bool>()
    }
}
impl HTMLElement {
    /// Getter of the `popover` attribute.
    /// [`HTMLElement.popover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/popover)
    pub fn popover(&self) -> String {
        self.inner.get("popover").as_::<String>()
    }

    /// Setter of the `popover` attribute.
    /// [`HTMLElement.popover`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/popover)
    pub fn set_popover(&mut self, value: &str) {
        self.inner.set("popover", value);
    }
}
impl HTMLElement {
    /// Getter of the `scrollParent` attribute.
    /// [`HTMLElement.scrollParent`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/scrollParent)
    pub fn scroll_parent(&self) -> Element {
        self.inner.get("scrollParent").as_::<Element>()
    }
}
impl HTMLElement {
    /// Getter of the `offsetParent` attribute.
    /// [`HTMLElement.offsetParent`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetParent)
    pub fn offset_parent(&self) -> Element {
        self.inner.get("offsetParent").as_::<Element>()
    }
}
impl HTMLElement {
    /// Getter of the `offsetTop` attribute.
    /// [`HTMLElement.offsetTop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetTop)
    pub fn offset_top(&self) -> i32 {
        self.inner.get("offsetTop").as_::<i32>()
    }
}
impl HTMLElement {
    /// Getter of the `offsetLeft` attribute.
    /// [`HTMLElement.offsetLeft`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetLeft)
    pub fn offset_left(&self) -> i32 {
        self.inner.get("offsetLeft").as_::<i32>()
    }
}
impl HTMLElement {
    /// Getter of the `offsetWidth` attribute.
    /// [`HTMLElement.offsetWidth`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetWidth)
    pub fn offset_width(&self) -> i32 {
        self.inner.get("offsetWidth").as_::<i32>()
    }
}
impl HTMLElement {
    /// Getter of the `offsetHeight` attribute.
    /// [`HTMLElement.offsetHeight`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetHeight)
    pub fn offset_height(&self) -> i32 {
        self.inner.get("offsetHeight").as_::<i32>()
    }
}
impl HTMLElement {
    /// Getter of the `editContext` attribute.
    /// [`HTMLElement.editContext`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/editContext)
    pub fn edit_context(&self) -> EditContext {
        self.inner.get("editContext").as_::<EditContext>()
    }

    /// Setter of the `editContext` attribute.
    /// [`HTMLElement.editContext`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/editContext)
    pub fn set_edit_context(&mut self, value: &EditContext) {
        self.inner.set("editContext", value);
    }
}
impl HTMLElement {
    /// Getter of the `style` attribute.
    /// [`HTMLElement.style`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/style)
    pub fn style(&self) -> CSSStyleDeclaration {
        self.inner.get("style").as_::<CSSStyleDeclaration>()
    }
}
impl HTMLElement {
    /// Getter of the `onbeforexrselect` attribute.
    /// [`HTMLElement.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onbeforexrselect)
    pub fn onbeforexrselect(&self) -> Any {
        self.inner.get("onbeforexrselect").as_::<Any>()
    }

    /// Setter of the `onbeforexrselect` attribute.
    /// [`HTMLElement.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onbeforexrselect)
    pub fn set_onbeforexrselect(&mut self, value: &Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
impl HTMLElement {
    /// Getter of the `virtualKeyboardPolicy` attribute.
    /// [`HTMLElement.virtualKeyboardPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/virtualKeyboardPolicy)
    pub fn virtual_keyboard_policy(&self) -> String {
        self.inner.get("virtualKeyboardPolicy").as_::<String>()
    }

    /// Setter of the `virtualKeyboardPolicy` attribute.
    /// [`HTMLElement.virtualKeyboardPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/virtualKeyboardPolicy)
    pub fn set_virtual_keyboard_policy(&mut self, value: &str) {
        self.inner.set("virtualKeyboardPolicy", value);
    }
}
impl HTMLElement {
    /// Getter of the `dataset` attribute.
    /// [`HTMLElement.dataset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dataset)
    pub fn dataset(&self) -> DOMStringMap {
        self.inner.get("dataset").as_::<DOMStringMap>()
    }
}
impl HTMLElement {
    /// Getter of the `nonce` attribute.
    /// [`HTMLElement.nonce`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/nonce)
    pub fn nonce(&self) -> String {
        self.inner.get("nonce").as_::<String>()
    }

    /// Setter of the `nonce` attribute.
    /// [`HTMLElement.nonce`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/nonce)
    pub fn set_nonce(&mut self, value: &str) {
        self.inner.set("nonce", value);
    }
}
impl HTMLElement {
    /// Getter of the `autofocus` attribute.
    /// [`HTMLElement.autofocus`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autofocus)
    pub fn autofocus(&self) -> bool {
        self.inner.get("autofocus").as_::<bool>()
    }

    /// Setter of the `autofocus` attribute.
    /// [`HTMLElement.autofocus`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autofocus)
    pub fn set_autofocus(&mut self, value: bool) {
        self.inner.set("autofocus", value);
    }
}
impl HTMLElement {
    /// Getter of the `tabIndex` attribute.
    /// [`HTMLElement.tabIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex)
    pub fn tab_index(&self) -> i32 {
        self.inner.get("tabIndex").as_::<i32>()
    }

    /// Setter of the `tabIndex` attribute.
    /// [`HTMLElement.tabIndex`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex)
    pub fn set_tab_index(&mut self, value: i32) {
        self.inner.set("tabIndex", value);
    }
}
impl HTMLElement {
    /// The focus method.
    /// [`HTMLElement.focus`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus)
    pub fn focus0(&self) -> Undefined {
        self.inner.call("focus", &[]).as_::<Undefined>()
    }
    /// The focus method.
    /// [`HTMLElement.focus`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus)
    pub fn focus1(&self, options: &FocusOptions) -> Undefined {
        self.inner
            .call("focus", &[options.into()])
            .as_::<Undefined>()
    }
}
impl HTMLElement {
    /// The blur method.
    /// [`HTMLElement.blur`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/blur)
    pub fn blur(&self) -> Undefined {
        self.inner.call("blur", &[]).as_::<Undefined>()
    }
}
