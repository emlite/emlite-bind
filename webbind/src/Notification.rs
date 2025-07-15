use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotificationAction {
    inner: emlite::Val,
}
impl FromVal for NotificationAction {
    fn from_val(v: &emlite::Val) -> Self {
        NotificationAction { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NotificationAction {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NotificationAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NotificationAction {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NotificationAction {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NotificationAction> for emlite::Val {
    fn from(s: NotificationAction) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&NotificationAction> for emlite::Val {
    fn from(s: &NotificationAction) -> emlite::Val {
        s.inner.clone()
    }
}

impl NotificationAction {
    pub fn action(&self) -> DOMString {
        self.inner.get("action").as_::<DOMString>()
    }

    pub fn set_action(&mut self, value: DOMString) {
        self.inner.set("action", value);
    }
}
impl NotificationAction {
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }

    pub fn set_title(&mut self, value: DOMString) {
        self.inner.set("title", value);
    }
}
impl NotificationAction {
    pub fn icon(&self) -> USVString {
        self.inner.get("icon").as_::<USVString>()
    }

    pub fn set_icon(&mut self, value: USVString) {
        self.inner.set("icon", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Notification {
    inner: EventTarget,
}
impl FromVal for Notification {
    fn from_val(v: &emlite::Val) -> Self {
        Notification {
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
impl core::ops::Deref for Notification {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Notification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Notification {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Notification {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Notification> for emlite::Val {
    fn from(s: Notification) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Notification> for emlite::Val {
    fn from(s: &Notification) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Notification);

impl Notification {
    pub fn new0(title: DOMString) -> Notification {
        Self {
            inner: emlite::Val::global("Notification")
                .new(&[title.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(title: DOMString, options: NotificationOptions) -> Notification {
        Self {
            inner: emlite::Val::global("Notification")
                .new(&[title.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl Notification {
    pub fn permission() -> NotificationPermission {
        emlite::Val::global("Notification")
            .get("permission")
            .as_::<NotificationPermission>()
    }
}
impl Notification {
    pub fn request_permission0() -> Promise {
        emlite::Val::global("Notification")
            .call("requestPermission", &[])
            .as_::<Promise>()
    }

    pub fn request_permission1(deprecated_callback: Function) -> Promise {
        emlite::Val::global("Notification")
            .call("requestPermission", &[deprecated_callback.into()])
            .as_::<Promise>()
    }
}
impl Notification {
    pub fn max_actions() -> u32 {
        emlite::Val::global("Notification")
            .get("maxActions")
            .as_::<u32>()
    }
}
impl Notification {
    pub fn onclick(&self) -> Any {
        self.inner.get("onclick").as_::<Any>()
    }

    pub fn set_onclick(&mut self, value: Any) {
        self.inner.set("onclick", value);
    }
}
impl Notification {
    pub fn onshow(&self) -> Any {
        self.inner.get("onshow").as_::<Any>()
    }

    pub fn set_onshow(&mut self, value: Any) {
        self.inner.set("onshow", value);
    }
}
impl Notification {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: Any) {
        self.inner.set("onerror", value);
    }
}
impl Notification {
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    pub fn set_onclose(&mut self, value: Any) {
        self.inner.set("onclose", value);
    }
}
impl Notification {
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }
}
impl Notification {
    pub fn dir(&self) -> NotificationDirection {
        self.inner.get("dir").as_::<NotificationDirection>()
    }
}
impl Notification {
    pub fn lang(&self) -> DOMString {
        self.inner.get("lang").as_::<DOMString>()
    }
}
impl Notification {
    pub fn body(&self) -> DOMString {
        self.inner.get("body").as_::<DOMString>()
    }
}
impl Notification {
    pub fn tag(&self) -> DOMString {
        self.inner.get("tag").as_::<DOMString>()
    }
}
impl Notification {
    pub fn image(&self) -> USVString {
        self.inner.get("image").as_::<USVString>()
    }
}
impl Notification {
    pub fn icon(&self) -> USVString {
        self.inner.get("icon").as_::<USVString>()
    }
}
impl Notification {
    pub fn badge(&self) -> USVString {
        self.inner.get("badge").as_::<USVString>()
    }
}
impl Notification {
    pub fn vibrate(&self) -> FrozenArray<u32> {
        self.inner.get("vibrate").as_::<FrozenArray<u32>>()
    }
}
impl Notification {
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }
}
impl Notification {
    pub fn renotify(&self) -> bool {
        self.inner.get("renotify").as_::<bool>()
    }
}
impl Notification {
    pub fn silent(&self) -> bool {
        self.inner.get("silent").as_::<bool>()
    }
}
impl Notification {
    pub fn require_interaction(&self) -> bool {
        self.inner.get("requireInteraction").as_::<bool>()
    }
}
impl Notification {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }
}
impl Notification {
    pub fn actions(&self) -> FrozenArray<NotificationAction> {
        self.inner
            .get("actions")
            .as_::<FrozenArray<NotificationAction>>()
    }
}
impl Notification {
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
