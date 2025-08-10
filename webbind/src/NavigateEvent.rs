use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationInterceptOptions {
    inner: Any,
}
impl FromVal for NavigationInterceptOptions {
    fn from_val(v: &Any) -> Self {
        NavigationInterceptOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationInterceptOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationInterceptOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationInterceptOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationInterceptOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationInterceptOptions> for Any {
    fn from(s: NavigationInterceptOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationInterceptOptions> for Any {
    fn from(s: &NavigationInterceptOptions) -> Any {
        s.inner.clone()
    }
}

impl NavigationInterceptOptions {
    pub fn handler(&self) -> Function {
        self.inner.get("handler").as_::<Function>()
    }

    pub fn set_handler(&mut self, value: &Function) {
        self.inner.set("handler", value);
    }
}
impl NavigationInterceptOptions {
    pub fn focus_reset(&self) -> NavigationFocusReset {
        self.inner.get("focusReset").as_::<NavigationFocusReset>()
    }

    pub fn set_focus_reset(&mut self, value: &NavigationFocusReset) {
        self.inner.set("focusReset", value);
    }
}
impl NavigationInterceptOptions {
    pub fn scroll(&self) -> NavigationScrollBehavior {
        self.inner.get("scroll").as_::<NavigationScrollBehavior>()
    }

    pub fn set_scroll(&mut self, value: &NavigationScrollBehavior) {
        self.inner.set("scroll", value);
    }
}
/// The NavigateEvent class.
/// [`NavigateEvent`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigateEvent {
    inner: Event,
}
impl FromVal for NavigateEvent {
    fn from_val(v: &Any) -> Self {
        NavigateEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for NavigateEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigateEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigateEvent> for Any {
    fn from(s: NavigateEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigateEvent> for Any {
    fn from(s: &NavigateEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigateEvent);

impl NavigateEvent {
    /// The `new NavigateEvent(..)` constructor, creating a new NavigateEvent instance
    pub fn new(type_: &JsString, event_init_dict: &NavigateEventInit) -> NavigateEvent {
        Self {
            inner: Any::global("NavigateEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl NavigateEvent {
    /// Getter of the `navigationType` attribute.
    /// [`NavigateEvent.navigationType`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/navigationType)
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }
}
impl NavigateEvent {
    /// Getter of the `destination` attribute.
    /// [`NavigateEvent.destination`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/destination)
    pub fn destination(&self) -> NavigationDestination {
        self.inner.get("destination").as_::<NavigationDestination>()
    }
}
impl NavigateEvent {
    /// Getter of the `canIntercept` attribute.
    /// [`NavigateEvent.canIntercept`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/canIntercept)
    pub fn can_intercept(&self) -> bool {
        self.inner.get("canIntercept").as_::<bool>()
    }
}
impl NavigateEvent {
    /// Getter of the `userInitiated` attribute.
    /// [`NavigateEvent.userInitiated`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/userInitiated)
    pub fn user_initiated(&self) -> bool {
        self.inner.get("userInitiated").as_::<bool>()
    }
}
impl NavigateEvent {
    /// Getter of the `hashChange` attribute.
    /// [`NavigateEvent.hashChange`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/hashChange)
    pub fn hash_change(&self) -> bool {
        self.inner.get("hashChange").as_::<bool>()
    }
}
impl NavigateEvent {
    /// Getter of the `signal` attribute.
    /// [`NavigateEvent.signal`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/signal)
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
impl NavigateEvent {
    /// Getter of the `formData` attribute.
    /// [`NavigateEvent.formData`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/formData)
    pub fn form_data(&self) -> FormData {
        self.inner.get("formData").as_::<FormData>()
    }
}
impl NavigateEvent {
    /// Getter of the `downloadRequest` attribute.
    /// [`NavigateEvent.downloadRequest`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/downloadRequest)
    pub fn download_request(&self) -> JsString {
        self.inner.get("downloadRequest").as_::<JsString>()
    }
}
impl NavigateEvent {
    /// Getter of the `info` attribute.
    /// [`NavigateEvent.info`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/info)
    pub fn info(&self) -> Any {
        self.inner.get("info").as_::<Any>()
    }
}
impl NavigateEvent {
    /// Getter of the `hasUAVisualTransition` attribute.
    /// [`NavigateEvent.hasUAVisualTransition`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/hasUAVisualTransition)
    pub fn has_ua_visual_transition(&self) -> bool {
        self.inner.get("hasUAVisualTransition").as_::<bool>()
    }
}
impl NavigateEvent {
    /// Getter of the `sourceElement` attribute.
    /// [`NavigateEvent.sourceElement`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/sourceElement)
    pub fn source_element(&self) -> Element {
        self.inner.get("sourceElement").as_::<Element>()
    }
}
impl NavigateEvent {
    /// The intercept method.
    /// [`NavigateEvent.intercept`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/intercept)
    pub fn intercept0(&self) -> Undefined {
        self.inner.call("intercept", &[]).as_::<Undefined>()
    }
    /// The intercept method.
    /// [`NavigateEvent.intercept`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/intercept)
    pub fn intercept1(&self, options: &NavigationInterceptOptions) -> Undefined {
        self.inner
            .call("intercept", &[options.into()])
            .as_::<Undefined>()
    }
}
impl NavigateEvent {
    /// The scroll method.
    /// [`NavigateEvent.scroll`](https://developer.mozilla.org/en-US/docs/Web/API/NavigateEvent/scroll)
    pub fn scroll(&self) -> Undefined {
        self.inner.call("scroll", &[]).as_::<Undefined>()
    }
}
