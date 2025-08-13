use super::*;




/// The ShadowRoot class.
/// [`ShadowRoot`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShadowRoot {
    inner: DocumentFragment,
}

impl FromVal for ShadowRoot {
    fn from_val(v: &Any) -> Self {
        ShadowRoot { inner: DocumentFragment::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ShadowRoot {
    type Target = DocumentFragment;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ShadowRoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ShadowRoot {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ShadowRoot {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ShadowRoot> for Any {
    fn from(s: ShadowRoot) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ShadowRoot> for Any {
    fn from(s: &ShadowRoot) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ShadowRoot);


impl ShadowRoot {
    /// Getter of the `mode` attribute.
    /// [`ShadowRoot.mode`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode)
    pub fn mode(&self) -> ShadowRootMode {
        self.inner.get("mode").as_::<ShadowRootMode>()
    }

}
impl ShadowRoot {
    /// Getter of the `delegatesFocus` attribute.
    /// [`ShadowRoot.delegatesFocus`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/delegatesFocus)
    pub fn delegates_focus(&self) -> bool {
        self.inner.get("delegatesFocus").as_::<bool>()
    }

}
impl ShadowRoot {
    /// Getter of the `slotAssignment` attribute.
    /// [`ShadowRoot.slotAssignment`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/slotAssignment)
    pub fn slot_assignment(&self) -> SlotAssignmentMode {
        self.inner.get("slotAssignment").as_::<SlotAssignmentMode>()
    }

}
impl ShadowRoot {
    /// Getter of the `clonable` attribute.
    /// [`ShadowRoot.clonable`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/clonable)
    pub fn clonable(&self) -> bool {
        self.inner.get("clonable").as_::<bool>()
    }

}
impl ShadowRoot {
    /// Getter of the `serializable` attribute.
    /// [`ShadowRoot.serializable`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/serializable)
    pub fn serializable(&self) -> bool {
        self.inner.get("serializable").as_::<bool>()
    }

}
impl ShadowRoot {
    /// Getter of the `host` attribute.
    /// [`ShadowRoot.host`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/host)
    pub fn host(&self) -> Element {
        self.inner.get("host").as_::<Element>()
    }

}
impl ShadowRoot {
    /// Getter of the `onslotchange` attribute.
    /// [`ShadowRoot.onslotchange`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/onslotchange)
    pub fn onslotchange(&self) -> Any {
        self.inner.get("onslotchange").as_::<Any>()
    }

    /// Setter of the `onslotchange` attribute.
    /// [`ShadowRoot.onslotchange`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/onslotchange)
    pub fn set_onslotchange(&mut self, value: &Any) {
        self.inner.set("onslotchange", value);
    }
}
impl ShadowRoot {
    /// The setHTMLUnsafe method.
    /// [`ShadowRoot.setHTMLUnsafe`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/setHTMLUnsafe)
    pub fn set_html_unsafe(&self, html: &Any) -> Undefined {
        self.inner.call("setHTMLUnsafe", &[html.into(), ]).as_::<Undefined>()
    }
}
impl ShadowRoot {
    /// The getHTML method.
    /// [`ShadowRoot.getHTML`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getHTML)
    pub fn get_html0(&self, ) -> JsString {
        self.inner.call("getHTML", &[]).as_::<JsString>()
    }
    /// The getHTML method.
    /// [`ShadowRoot.getHTML`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getHTML)
    pub fn get_html1(&self, options: &GetHTMLOptions) -> JsString {
        self.inner.call("getHTML", &[options.into(), ]).as_::<JsString>()
    }
}
impl ShadowRoot {
    /// Getter of the `innerHTML` attribute.
    /// [`ShadowRoot.innerHTML`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)
    pub fn inner_html(&self) -> Any {
        self.inner.get("innerHTML").as_::<Any>()
    }

    /// Setter of the `innerHTML` attribute.
    /// [`ShadowRoot.innerHTML`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)
    pub fn set_inner_html(&mut self, value: &Any) {
        self.inner.set("innerHTML", value);
    }
}
impl ShadowRoot {
    /// The getAnimations method.
    /// [`ShadowRoot.getAnimations`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getAnimations)
    pub fn get_animations(&self, ) -> TypedArray<Animation> {
        self.inner.call("getAnimations", &[]).as_::<TypedArray<Animation>>()
    }
}
