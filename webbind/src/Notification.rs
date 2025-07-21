use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotificationAction {
    inner: Any,
}
impl FromVal for NotificationAction {
    fn from_val(v: &Any) -> Self {
        NotificationAction { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NotificationAction {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NotificationAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NotificationAction {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NotificationAction {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NotificationAction> for Any {
    fn from(s: NotificationAction) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NotificationAction> for Any {
    fn from(s: &NotificationAction) -> Any {
        s.inner.clone()
    }
}

impl NotificationAction {
    pub fn action(&self) -> String {
        self.inner.get("action").as_::<String>()
    }

    pub fn set_action(&mut self, value: &str) {
        self.inner.set("action", value);
    }
}
impl NotificationAction {
    pub fn title(&self) -> String {
        self.inner.get("title").as_::<String>()
    }

    pub fn set_title(&mut self, value: &str) {
        self.inner.set("title", value);
    }
}
impl NotificationAction {
    pub fn icon(&self) -> String {
        self.inner.get("icon").as_::<String>()
    }

    pub fn set_icon(&mut self, value: &str) {
        self.inner.set("icon", value);
    }
}
/// The Notification class.
/// [`Notification`](https://developer.mozilla.org/en-US/docs/Web/API/Notification)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Notification {
    inner: EventTarget,
}
impl FromVal for Notification {
    fn from_val(v: &Any) -> Self {
        Notification {
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
impl AsRef<Any> for Notification {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Notification {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Notification> for Any {
    fn from(s: Notification) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Notification> for Any {
    fn from(s: &Notification) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Notification);

impl Notification {
    /// The `new Notification(..)` constructor, creating a new Notification instance
    pub fn new0(title: &str) -> Notification {
        Self {
            inner: Any::global("Notification")
                .new(&[title.into()])
                .as_::<EventTarget>(),
        }
    }

    /// The `new Notification(..)` constructor, creating a new Notification instance
    pub fn new1(title: &str, options: &NotificationOptions) -> Notification {
        Self {
            inner: Any::global("Notification")
                .new(&[title.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl Notification {
    /// Getter of the `permission` static attribute.
    /// [`Notification.permission`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/permission)
    pub fn permission() -> NotificationPermission {
        Any::global("Notification")
            .get("permission")
            .as_::<NotificationPermission>()
    }
}
impl Notification {
    /// The requestPermission method.
    /// [`Notification.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission)
    pub fn request_permission0() -> Promise<NotificationPermission> {
        Any::global("Notification")
            .call("requestPermission", &[])
            .as_::<Promise<NotificationPermission>>()
    }
    /// The requestPermission method.
    /// [`Notification.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission)
    pub fn request_permission1(deprecated_callback: &Function) -> Promise<NotificationPermission> {
        Any::global("Notification")
            .call("requestPermission", &[deprecated_callback.into()])
            .as_::<Promise<NotificationPermission>>()
    }
}
impl Notification {
    /// Getter of the `maxActions` static attribute.
    /// [`Notification.maxActions`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/maxActions)
    pub fn max_actions() -> u32 {
        Any::global("Notification").get("maxActions").as_::<u32>()
    }
}
impl Notification {
    /// Getter of the `onclick` attribute.
    /// [`Notification.onclick`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)
    pub fn onclick(&self) -> Any {
        self.inner.get("onclick").as_::<Any>()
    }

    /// Setter of the `onclick` attribute.
    /// [`Notification.onclick`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)
    pub fn set_onclick(&mut self, value: &Any) {
        self.inner.set("onclick", value);
    }
}
impl Notification {
    /// Getter of the `onshow` attribute.
    /// [`Notification.onshow`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)
    pub fn onshow(&self) -> Any {
        self.inner.get("onshow").as_::<Any>()
    }

    /// Setter of the `onshow` attribute.
    /// [`Notification.onshow`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)
    pub fn set_onshow(&mut self, value: &Any) {
        self.inner.set("onshow", value);
    }
}
impl Notification {
    /// Getter of the `onerror` attribute.
    /// [`Notification.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`Notification.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl Notification {
    /// Getter of the `onclose` attribute.
    /// [`Notification.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    /// Setter of the `onclose` attribute.
    /// [`Notification.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)
    pub fn set_onclose(&mut self, value: &Any) {
        self.inner.set("onclose", value);
    }
}
impl Notification {
    /// Getter of the `title` attribute.
    /// [`Notification.title`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/title)
    pub fn title(&self) -> String {
        self.inner.get("title").as_::<String>()
    }
}
impl Notification {
    /// Getter of the `dir` attribute.
    /// [`Notification.dir`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/dir)
    pub fn dir(&self) -> NotificationDirection {
        self.inner.get("dir").as_::<NotificationDirection>()
    }
}
impl Notification {
    /// Getter of the `lang` attribute.
    /// [`Notification.lang`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/lang)
    pub fn lang(&self) -> String {
        self.inner.get("lang").as_::<String>()
    }
}
impl Notification {
    /// Getter of the `body` attribute.
    /// [`Notification.body`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/body)
    pub fn body(&self) -> String {
        self.inner.get("body").as_::<String>()
    }
}
impl Notification {
    /// Getter of the `tag` attribute.
    /// [`Notification.tag`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/tag)
    pub fn tag(&self) -> String {
        self.inner.get("tag").as_::<String>()
    }
}
impl Notification {
    /// Getter of the `image` attribute.
    /// [`Notification.image`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/image)
    pub fn image(&self) -> String {
        self.inner.get("image").as_::<String>()
    }
}
impl Notification {
    /// Getter of the `icon` attribute.
    /// [`Notification.icon`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/icon)
    pub fn icon(&self) -> String {
        self.inner.get("icon").as_::<String>()
    }
}
impl Notification {
    /// Getter of the `badge` attribute.
    /// [`Notification.badge`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/badge)
    pub fn badge(&self) -> String {
        self.inner.get("badge").as_::<String>()
    }
}
impl Notification {
    /// Getter of the `vibrate` attribute.
    /// [`Notification.vibrate`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/vibrate)
    pub fn vibrate(&self) -> FrozenArray<u32> {
        self.inner.get("vibrate").as_::<FrozenArray<u32>>()
    }
}
impl Notification {
    /// Getter of the `timestamp` attribute.
    /// [`Notification.timestamp`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/timestamp)
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }
}
impl Notification {
    /// Getter of the `renotify` attribute.
    /// [`Notification.renotify`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/renotify)
    pub fn renotify(&self) -> bool {
        self.inner.get("renotify").as_::<bool>()
    }
}
impl Notification {
    /// Getter of the `silent` attribute.
    /// [`Notification.silent`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/silent)
    pub fn silent(&self) -> bool {
        self.inner.get("silent").as_::<bool>()
    }
}
impl Notification {
    /// Getter of the `requireInteraction` attribute.
    /// [`Notification.requireInteraction`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requireInteraction)
    pub fn require_interaction(&self) -> bool {
        self.inner.get("requireInteraction").as_::<bool>()
    }
}
impl Notification {
    /// Getter of the `data` attribute.
    /// [`Notification.data`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/data)
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }
}
impl Notification {
    /// Getter of the `actions` attribute.
    /// [`Notification.actions`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/actions)
    pub fn actions(&self) -> FrozenArray<NotificationAction> {
        self.inner
            .get("actions")
            .as_::<FrozenArray<NotificationAction>>()
    }
}
impl Notification {
    /// The close method.
    /// [`Notification.close`](https://developer.mozilla.org/en-US/docs/Web/API/Notification/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
