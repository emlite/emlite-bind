use super::*;

/// The EditContext class.
/// [`EditContext`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EditContext {
    inner: EventTarget,
}
impl FromVal for EditContext {
    fn from_val(v: &Any) -> Self {
        EditContext {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for EditContext {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EditContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for EditContext {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for EditContext {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<EditContext> for Any {
    fn from(s: EditContext) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&EditContext> for Any {
    fn from(s: &EditContext) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(EditContext);

impl EditContext {
    /// The `new EditContext(..)` constructor, creating a new EditContext instance
    pub fn new0() -> EditContext {
        Self {
            inner: Any::global("EditContext").new(&[]).as_::<EventTarget>(),
        }
    }

    /// The `new EditContext(..)` constructor, creating a new EditContext instance
    pub fn new1(options: &Any) -> EditContext {
        Self {
            inner: Any::global("EditContext")
                .new(&[options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl EditContext {
    /// The updateText method.
    /// [`EditContext.updateText`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/updateText)
    pub fn update_text(&self, range_start: u32, range_end: u32, text: &JsString) -> Undefined {
        self.inner
            .call(
                "updateText",
                &[range_start.into(), range_end.into(), text.into()],
            )
            .as_::<Undefined>()
    }
}
impl EditContext {
    /// The updateSelection method.
    /// [`EditContext.updateSelection`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/updateSelection)
    pub fn update_selection(&self, start: u32, end: u32) -> Undefined {
        self.inner
            .call("updateSelection", &[start.into(), end.into()])
            .as_::<Undefined>()
    }
}
impl EditContext {
    /// The updateControlBounds method.
    /// [`EditContext.updateControlBounds`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/updateControlBounds)
    pub fn update_control_bounds(&self, control_bounds: &DOMRect) -> Undefined {
        self.inner
            .call("updateControlBounds", &[control_bounds.into()])
            .as_::<Undefined>()
    }
}
impl EditContext {
    /// The updateSelectionBounds method.
    /// [`EditContext.updateSelectionBounds`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/updateSelectionBounds)
    pub fn update_selection_bounds(&self, selection_bounds: &DOMRect) -> Undefined {
        self.inner
            .call("updateSelectionBounds", &[selection_bounds.into()])
            .as_::<Undefined>()
    }
}
impl EditContext {
    /// The updateCharacterBounds method.
    /// [`EditContext.updateCharacterBounds`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/updateCharacterBounds)
    pub fn update_character_bounds(
        &self,
        range_start: u32,
        character_bounds: &TypedArray<DOMRect>,
    ) -> Undefined {
        self.inner
            .call(
                "updateCharacterBounds",
                &[range_start.into(), character_bounds.into()],
            )
            .as_::<Undefined>()
    }
}
impl EditContext {
    /// The attachedElements method.
    /// [`EditContext.attachedElements`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/attachedElements)
    pub fn attached_elements(&self) -> TypedArray<HTMLElement> {
        self.inner
            .call("attachedElements", &[])
            .as_::<TypedArray<HTMLElement>>()
    }
}
impl EditContext {
    /// Getter of the `text` attribute.
    /// [`EditContext.text`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/text)
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }
}
impl EditContext {
    /// Getter of the `selectionStart` attribute.
    /// [`EditContext.selectionStart`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/selectionStart)
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }
}
impl EditContext {
    /// Getter of the `selectionEnd` attribute.
    /// [`EditContext.selectionEnd`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/selectionEnd)
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }
}
impl EditContext {
    /// Getter of the `characterBoundsRangeStart` attribute.
    /// [`EditContext.characterBoundsRangeStart`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/characterBoundsRangeStart)
    pub fn character_bounds_range_start(&self) -> u32 {
        self.inner.get("characterBoundsRangeStart").as_::<u32>()
    }
}
impl EditContext {
    /// The characterBounds method.
    /// [`EditContext.characterBounds`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/characterBounds)
    pub fn character_bounds(&self) -> TypedArray<DOMRect> {
        self.inner
            .call("characterBounds", &[])
            .as_::<TypedArray<DOMRect>>()
    }
}
impl EditContext {
    /// Getter of the `ontextupdate` attribute.
    /// [`EditContext.ontextupdate`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/ontextupdate)
    pub fn ontextupdate(&self) -> Any {
        self.inner.get("ontextupdate").as_::<Any>()
    }

    /// Setter of the `ontextupdate` attribute.
    /// [`EditContext.ontextupdate`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/ontextupdate)
    pub fn set_ontextupdate(&mut self, value: &Any) {
        self.inner.set("ontextupdate", value);
    }
}
impl EditContext {
    /// Getter of the `ontextformatupdate` attribute.
    /// [`EditContext.ontextformatupdate`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/ontextformatupdate)
    pub fn ontextformatupdate(&self) -> Any {
        self.inner.get("ontextformatupdate").as_::<Any>()
    }

    /// Setter of the `ontextformatupdate` attribute.
    /// [`EditContext.ontextformatupdate`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/ontextformatupdate)
    pub fn set_ontextformatupdate(&mut self, value: &Any) {
        self.inner.set("ontextformatupdate", value);
    }
}
impl EditContext {
    /// Getter of the `oncharacterboundsupdate` attribute.
    /// [`EditContext.oncharacterboundsupdate`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/oncharacterboundsupdate)
    pub fn oncharacterboundsupdate(&self) -> Any {
        self.inner.get("oncharacterboundsupdate").as_::<Any>()
    }

    /// Setter of the `oncharacterboundsupdate` attribute.
    /// [`EditContext.oncharacterboundsupdate`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/oncharacterboundsupdate)
    pub fn set_oncharacterboundsupdate(&mut self, value: &Any) {
        self.inner.set("oncharacterboundsupdate", value);
    }
}
impl EditContext {
    /// Getter of the `oncompositionstart` attribute.
    /// [`EditContext.oncompositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/oncompositionstart)
    pub fn oncompositionstart(&self) -> Any {
        self.inner.get("oncompositionstart").as_::<Any>()
    }

    /// Setter of the `oncompositionstart` attribute.
    /// [`EditContext.oncompositionstart`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/oncompositionstart)
    pub fn set_oncompositionstart(&mut self, value: &Any) {
        self.inner.set("oncompositionstart", value);
    }
}
impl EditContext {
    /// Getter of the `oncompositionend` attribute.
    /// [`EditContext.oncompositionend`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/oncompositionend)
    pub fn oncompositionend(&self) -> Any {
        self.inner.get("oncompositionend").as_::<Any>()
    }

    /// Setter of the `oncompositionend` attribute.
    /// [`EditContext.oncompositionend`](https://developer.mozilla.org/en-US/docs/Web/API/EditContext/oncompositionend)
    pub fn set_oncompositionend(&mut self, value: &Any) {
        self.inner.set("oncompositionend", value);
    }
}
