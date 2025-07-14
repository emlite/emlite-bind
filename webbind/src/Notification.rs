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

impl NotificationAction {
    pub fn action(&self) -> jsbind::DOMString {
        self.inner.get("action").as_::<jsbind::DOMString>()
    }

    pub fn set_action(&mut self, value: jsbind::DOMString) {
        self.inner.set("action", value);
    }
}
impl NotificationAction {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }

    pub fn set_title(&mut self, value: jsbind::DOMString) {
        self.inner.set("title", value);
    }
}
impl NotificationAction {
    pub fn icon(&self) -> jsbind::USVString {
        self.inner.get("icon").as_::<jsbind::USVString>()
    }

    pub fn set_icon(&mut self, value: jsbind::USVString) {
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
jsbind::utils::impl_dyn_cast!(Notification);

impl Notification {
    pub fn new0(title: jsbind::DOMString) -> Notification {
        Self {
            inner: emlite::Val::global("Notification")
                .new(&[title.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(title: jsbind::DOMString, options: NotificationOptions) -> Notification {
        Self {
            inner: emlite::Val::global("Notification")
                .new(&[title.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl Notification {
    pub fn permission() -> NotificationPermission {
        emlite::Val::global("notification")
            .get("permission")
            .as_::<NotificationPermission>()
    }
}
impl Notification {
    pub fn request_permission0() -> jsbind::Promise {
        emlite::Val::global("notification")
            .call("requestPermission", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_permission1(deprecated_callback: jsbind::Function) -> jsbind::Promise {
        emlite::Val::global("notification")
            .call("requestPermission", &[deprecated_callback.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Notification {
    pub fn max_actions() -> u32 {
        emlite::Val::global("notification")
            .get("maxActions")
            .as_::<u32>()
    }
}
impl Notification {
    pub fn onclick(&self) -> jsbind::Any {
        self.inner.get("onclick").as_::<jsbind::Any>()
    }

    pub fn set_onclick(&mut self, value: jsbind::Any) {
        self.inner.set("onclick", value);
    }
}
impl Notification {
    pub fn onshow(&self) -> jsbind::Any {
        self.inner.get("onshow").as_::<jsbind::Any>()
    }

    pub fn set_onshow(&mut self, value: jsbind::Any) {
        self.inner.set("onshow", value);
    }
}
impl Notification {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl Notification {
    pub fn onclose(&self) -> jsbind::Any {
        self.inner.get("onclose").as_::<jsbind::Any>()
    }

    pub fn set_onclose(&mut self, value: jsbind::Any) {
        self.inner.set("onclose", value);
    }
}
impl Notification {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }
}
impl Notification {
    pub fn dir(&self) -> NotificationDirection {
        self.inner.get("dir").as_::<NotificationDirection>()
    }
}
impl Notification {
    pub fn lang(&self) -> jsbind::DOMString {
        self.inner.get("lang").as_::<jsbind::DOMString>()
    }
}
impl Notification {
    pub fn body(&self) -> jsbind::DOMString {
        self.inner.get("body").as_::<jsbind::DOMString>()
    }
}
impl Notification {
    pub fn tag(&self) -> jsbind::DOMString {
        self.inner.get("tag").as_::<jsbind::DOMString>()
    }
}
impl Notification {
    pub fn image(&self) -> jsbind::USVString {
        self.inner.get("image").as_::<jsbind::USVString>()
    }
}
impl Notification {
    pub fn icon(&self) -> jsbind::USVString {
        self.inner.get("icon").as_::<jsbind::USVString>()
    }
}
impl Notification {
    pub fn badge(&self) -> jsbind::USVString {
        self.inner.get("badge").as_::<jsbind::USVString>()
    }
}
impl Notification {
    pub fn vibrate(&self) -> jsbind::FrozenArray<u32> {
        self.inner.get("vibrate").as_::<jsbind::FrozenArray<u32>>()
    }
}
impl Notification {
    pub fn timestamp(&self) -> jsbind::Any {
        self.inner.get("timestamp").as_::<jsbind::Any>()
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
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }
}
impl Notification {
    pub fn actions(&self) -> jsbind::FrozenArray<NotificationAction> {
        self.inner
            .get("actions")
            .as_::<jsbind::FrozenArray<NotificationAction>>()
    }
}
impl Notification {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
