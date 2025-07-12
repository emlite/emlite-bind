use super::*;

#[derive(Clone, Debug)]
pub struct EditContext {
    inner: EventTarget,
}
impl FromVal for EditContext {
    fn from_val(v: &emlite::Val) -> Self {
        EditContext {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for EditContext {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EditContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EditContext> for emlite::Val {
    fn from(s: EditContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EditContext {
    pub fn new0() -> EditContext {
        Self {
            inner: emlite::Val::global("EditContext")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(options: jsbind::Any) -> EditContext {
        Self {
            inner: emlite::Val::global("EditContext")
                .new(&[options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl EditContext {
    pub fn update_text(
        &self,
        range_start: u32,
        range_end: u32,
        text: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "updateText",
                &[range_start.into(), range_end.into(), text.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl EditContext {
    pub fn update_selection(&self, start: u32, end: u32) -> jsbind::Undefined {
        self.inner
            .call("updateSelection", &[start.into(), end.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl EditContext {
    pub fn update_control_bounds(&self, control_bounds: DOMRect) -> jsbind::Undefined {
        self.inner
            .call("updateControlBounds", &[control_bounds.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl EditContext {
    pub fn update_selection_bounds(&self, selection_bounds: DOMRect) -> jsbind::Undefined {
        self.inner
            .call("updateSelectionBounds", &[selection_bounds.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl EditContext {
    pub fn update_character_bounds(
        &self,
        range_start: u32,
        character_bounds: jsbind::Sequence<DOMRect>,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "updateCharacterBounds",
                &[range_start.into(), character_bounds.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl EditContext {
    pub fn attached_elements(&self) -> jsbind::Sequence<HTMLElement> {
        self.inner
            .call("attachedElements", &[])
            .as_::<jsbind::Sequence<HTMLElement>>()
    }
}
impl EditContext {
    pub fn text(&self) -> jsbind::DOMString {
        self.inner.get("text").as_::<jsbind::DOMString>()
    }
}
impl EditContext {
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }
}
impl EditContext {
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }
}
impl EditContext {
    pub fn character_bounds_range_start(&self) -> u32 {
        self.inner.get("characterBoundsRangeStart").as_::<u32>()
    }
}
impl EditContext {
    pub fn character_bounds(&self) -> jsbind::Sequence<DOMRect> {
        self.inner
            .call("characterBounds", &[])
            .as_::<jsbind::Sequence<DOMRect>>()
    }
}
impl EditContext {
    pub fn ontextupdate(&self) -> jsbind::Any {
        self.inner.get("ontextupdate").as_::<jsbind::Any>()
    }

    pub fn set_ontextupdate(&mut self, value: jsbind::Any) {
        self.inner.set("ontextupdate", value);
    }
}
impl EditContext {
    pub fn ontextformatupdate(&self) -> jsbind::Any {
        self.inner.get("ontextformatupdate").as_::<jsbind::Any>()
    }

    pub fn set_ontextformatupdate(&mut self, value: jsbind::Any) {
        self.inner.set("ontextformatupdate", value);
    }
}
impl EditContext {
    pub fn oncharacterboundsupdate(&self) -> jsbind::Any {
        self.inner
            .get("oncharacterboundsupdate")
            .as_::<jsbind::Any>()
    }

    pub fn set_oncharacterboundsupdate(&mut self, value: jsbind::Any) {
        self.inner.set("oncharacterboundsupdate", value);
    }
}
impl EditContext {
    pub fn oncompositionstart(&self) -> jsbind::Any {
        self.inner.get("oncompositionstart").as_::<jsbind::Any>()
    }

    pub fn set_oncompositionstart(&mut self, value: jsbind::Any) {
        self.inner.set("oncompositionstart", value);
    }
}
impl EditContext {
    pub fn oncompositionend(&self) -> jsbind::Any {
        self.inner.get("oncompositionend").as_::<jsbind::Any>()
    }

    pub fn set_oncompositionend(&mut self, value: jsbind::Any) {
        self.inner.set("oncompositionend", value);
    }
}
