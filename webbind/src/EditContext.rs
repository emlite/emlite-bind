use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EditContext {
    inner: EventTarget,
}
impl FromVal for EditContext {
    fn from_val(v: &emlite::Val) -> Self {
        EditContext { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for EditContext {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for EditContext {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<EditContext> for emlite::Val {
    fn from(s: EditContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(EditContext);



impl EditContext {
    pub fn new0() -> EditContext {
        Self {
            inner: emlite::Val::global("EditContext").new(&[]).as_::<EventTarget>(),
        }
    }

    pub fn new1(options: Any) -> EditContext {
        Self {
            inner: emlite::Val::global("EditContext").new(&[options.into()]).as_::<EventTarget>(),
        }
    }

}
impl EditContext {
    pub fn update_text(&self, range_start: u32, range_end: u32, text: DOMString) -> Undefined {
        self.inner.call("updateText", &[range_start.into(), range_end.into(), text.into(), ]).as_::<Undefined>()
    }

}
impl EditContext {
    pub fn update_selection(&self, start: u32, end: u32) -> Undefined {
        self.inner.call("updateSelection", &[start.into(), end.into(), ]).as_::<Undefined>()
    }

}
impl EditContext {
    pub fn update_control_bounds(&self, control_bounds: DOMRect) -> Undefined {
        self.inner.call("updateControlBounds", &[control_bounds.into(), ]).as_::<Undefined>()
    }

}
impl EditContext {
    pub fn update_selection_bounds(&self, selection_bounds: DOMRect) -> Undefined {
        self.inner.call("updateSelectionBounds", &[selection_bounds.into(), ]).as_::<Undefined>()
    }

}
impl EditContext {
    pub fn update_character_bounds(&self, range_start: u32, character_bounds: Sequence<DOMRect>) -> Undefined {
        self.inner.call("updateCharacterBounds", &[range_start.into(), character_bounds.into(), ]).as_::<Undefined>()
    }

}
impl EditContext {
    pub fn attached_elements(&self, ) -> Sequence<HTMLElement> {
        self.inner.call("attachedElements", &[]).as_::<Sequence<HTMLElement>>()
    }

}
impl EditContext {
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
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
    pub fn character_bounds(&self, ) -> Sequence<DOMRect> {
        self.inner.call("characterBounds", &[]).as_::<Sequence<DOMRect>>()
    }

}
impl EditContext {
    pub fn ontextupdate(&self) -> Any {
        self.inner.get("ontextupdate").as_::<Any>()
    }

    pub fn set_ontextupdate(&mut self, value: Any) {
        self.inner.set("ontextupdate", value);
    }

}
impl EditContext {
    pub fn ontextformatupdate(&self) -> Any {
        self.inner.get("ontextformatupdate").as_::<Any>()
    }

    pub fn set_ontextformatupdate(&mut self, value: Any) {
        self.inner.set("ontextformatupdate", value);
    }

}
impl EditContext {
    pub fn oncharacterboundsupdate(&self) -> Any {
        self.inner.get("oncharacterboundsupdate").as_::<Any>()
    }

    pub fn set_oncharacterboundsupdate(&mut self, value: Any) {
        self.inner.set("oncharacterboundsupdate", value);
    }

}
impl EditContext {
    pub fn oncompositionstart(&self) -> Any {
        self.inner.get("oncompositionstart").as_::<Any>()
    }

    pub fn set_oncompositionstart(&mut self, value: Any) {
        self.inner.set("oncompositionstart", value);
    }

}
impl EditContext {
    pub fn oncompositionend(&self) -> Any {
        self.inner.get("oncompositionend").as_::<Any>()
    }

    pub fn set_oncompositionend(&mut self, value: Any) {
        self.inner.set("oncompositionend", value);
    }

}
