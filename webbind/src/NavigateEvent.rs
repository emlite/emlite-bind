use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationInterceptOptions {
    inner: emlite::Val,
}
impl FromVal for NavigationInterceptOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationInterceptOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationInterceptOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationInterceptOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NavigationInterceptOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigationInterceptOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<NavigationInterceptOptions> for emlite::Val {
    fn from(s: NavigationInterceptOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationInterceptOptions {
    pub fn handler(&self) -> Function {
        self.inner.get("handler").as_::<Function>()
    }

    pub fn set_handler(&mut self, value: Function) {
        self.inner.set("handler", value);
    }

}
impl NavigationInterceptOptions {
    pub fn focus_reset(&self) -> NavigationFocusReset {
        self.inner.get("focusReset").as_::<NavigationFocusReset>()
    }

    pub fn set_focus_reset(&mut self, value: NavigationFocusReset) {
        self.inner.set("focusReset", value);
    }

}
impl NavigationInterceptOptions {
    pub fn scroll(&self) -> NavigationScrollBehavior {
        self.inner.get("scroll").as_::<NavigationScrollBehavior>()
    }

    pub fn set_scroll(&mut self, value: NavigationScrollBehavior) {
        self.inner.set("scroll", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigateEvent {
    inner: Event,
}
impl FromVal for NavigateEvent {
    fn from_val(v: &emlite::Val) -> Self {
        NavigateEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NavigateEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigateEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<NavigateEvent> for emlite::Val {
    fn from(s: NavigateEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NavigateEvent);



impl NavigateEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> NavigateEvent {
        Self {
            inner: emlite::Val::global("NavigateEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl NavigateEvent {
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }

}
impl NavigateEvent {
    pub fn destination(&self) -> NavigationDestination {
        self.inner.get("destination").as_::<NavigationDestination>()
    }

}
impl NavigateEvent {
    pub fn can_intercept(&self) -> bool {
        self.inner.get("canIntercept").as_::<bool>()
    }

}
impl NavigateEvent {
    pub fn user_initiated(&self) -> bool {
        self.inner.get("userInitiated").as_::<bool>()
    }

}
impl NavigateEvent {
    pub fn hash_change(&self) -> bool {
        self.inner.get("hashChange").as_::<bool>()
    }

}
impl NavigateEvent {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

}
impl NavigateEvent {
    pub fn form_data(&self) -> FormData {
        self.inner.get("formData").as_::<FormData>()
    }

}
impl NavigateEvent {
    pub fn download_request(&self) -> DOMString {
        self.inner.get("downloadRequest").as_::<DOMString>()
    }

}
impl NavigateEvent {
    pub fn info(&self) -> Any {
        self.inner.get("info").as_::<Any>()
    }

}
impl NavigateEvent {
    pub fn has_ua_visual_transition(&self) -> bool {
        self.inner.get("hasUAVisualTransition").as_::<bool>()
    }

}
impl NavigateEvent {
    pub fn source_element(&self) -> Element {
        self.inner.get("sourceElement").as_::<Element>()
    }

}
impl NavigateEvent {
    pub fn intercept0(&self, ) -> Undefined {
        self.inner.call("intercept", &[]).as_::<Undefined>()
    }

    pub fn intercept1(&self, options: NavigationInterceptOptions) -> Undefined {
        self.inner.call("intercept", &[options.into(), ]).as_::<Undefined>()
    }

}
impl NavigateEvent {
    pub fn scroll(&self, ) -> Undefined {
        self.inner.call("scroll", &[]).as_::<Undefined>()
    }

}
