use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigateEventInit {
    inner: Any,
}
impl FromVal for NavigateEventInit {
    fn from_val(v: &Any) -> Self {
        NavigateEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigateEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigateEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigateEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigateEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigateEventInit> for Any {
    fn from(s: NavigateEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigateEventInit> for Any {
    fn from(s: &NavigateEventInit) -> Any {
        s.inner.clone()
    }
}

impl NavigateEventInit {
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }

    pub fn set_navigation_type(&mut self, value: &NavigationType) {
        self.inner.set("navigationType", value);
    }
}
impl NavigateEventInit {
    pub fn destination(&self) -> NavigationDestination {
        self.inner.get("destination").as_::<NavigationDestination>()
    }

    pub fn set_destination(&mut self, value: &NavigationDestination) {
        self.inner.set("destination", value);
    }
}
impl NavigateEventInit {
    pub fn can_intercept(&self) -> bool {
        self.inner.get("canIntercept").as_::<bool>()
    }

    pub fn set_can_intercept(&mut self, value: bool) {
        self.inner.set("canIntercept", value);
    }
}
impl NavigateEventInit {
    pub fn user_initiated(&self) -> bool {
        self.inner.get("userInitiated").as_::<bool>()
    }

    pub fn set_user_initiated(&mut self, value: bool) {
        self.inner.set("userInitiated", value);
    }
}
impl NavigateEventInit {
    pub fn hash_change(&self) -> bool {
        self.inner.get("hashChange").as_::<bool>()
    }

    pub fn set_hash_change(&mut self, value: bool) {
        self.inner.set("hashChange", value);
    }
}
impl NavigateEventInit {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl NavigateEventInit {
    pub fn form_data(&self) -> FormData {
        self.inner.get("formData").as_::<FormData>()
    }

    pub fn set_form_data(&mut self, value: &FormData) {
        self.inner.set("formData", value);
    }
}
impl NavigateEventInit {
    pub fn download_request(&self) -> JsString {
        self.inner.get("downloadRequest").as_::<JsString>()
    }

    pub fn set_download_request(&mut self, value: &JsString) {
        self.inner.set("downloadRequest", value);
    }
}
impl NavigateEventInit {
    pub fn info(&self) -> Any {
        self.inner.get("info").as_::<Any>()
    }

    pub fn set_info(&mut self, value: &Any) {
        self.inner.set("info", value);
    }
}
impl NavigateEventInit {
    pub fn has_ua_visual_transition(&self) -> bool {
        self.inner.get("hasUAVisualTransition").as_::<bool>()
    }

    pub fn set_has_ua_visual_transition(&mut self, value: bool) {
        self.inner.set("hasUAVisualTransition", value);
    }
}
impl NavigateEventInit {
    pub fn source_element(&self) -> Element {
        self.inner.get("sourceElement").as_::<Element>()
    }

    pub fn set_source_element(&mut self, value: &Element) {
        self.inner.set("sourceElement", value);
    }
}
