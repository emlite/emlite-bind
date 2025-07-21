use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockOptions {
    inner: Any,
}
impl FromVal for LockOptions {
    fn from_val(v: &Any) -> Self {
        LockOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LockOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LockOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LockOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LockOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LockOptions> for Any {
    fn from(s: LockOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LockOptions> for Any {
    fn from(s: &LockOptions) -> Any {
        s.inner.clone()
    }
}

impl LockOptions {
    pub fn mode(&self) -> LockMode {
        self.inner.get("mode").as_::<LockMode>()
    }

    pub fn set_mode(&mut self, value: &LockMode) {
        self.inner.set("mode", value);
    }
}
impl LockOptions {
    pub fn if_available(&self) -> bool {
        self.inner.get("ifAvailable").as_::<bool>()
    }

    pub fn set_if_available(&mut self, value: bool) {
        self.inner.set("ifAvailable", value);
    }
}
impl LockOptions {
    pub fn steal(&self) -> bool {
        self.inner.get("steal").as_::<bool>()
    }

    pub fn set_steal(&mut self, value: bool) {
        self.inner.set("steal", value);
    }
}
impl LockOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockManagerSnapshot {
    inner: Any,
}
impl FromVal for LockManagerSnapshot {
    fn from_val(v: &Any) -> Self {
        LockManagerSnapshot { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LockManagerSnapshot {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LockManagerSnapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LockManagerSnapshot {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LockManagerSnapshot {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LockManagerSnapshot> for Any {
    fn from(s: LockManagerSnapshot) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LockManagerSnapshot> for Any {
    fn from(s: &LockManagerSnapshot) -> Any {
        s.inner.clone()
    }
}

impl LockManagerSnapshot {
    pub fn held(&self) -> Sequence<Any> {
        self.inner.get("held").as_::<Sequence<Any>>()
    }

    pub fn set_held(&mut self, value: &Sequence<Any>) {
        self.inner.set("held", value);
    }
}
impl LockManagerSnapshot {
    pub fn pending(&self) -> Sequence<Any> {
        self.inner.get("pending").as_::<Sequence<Any>>()
    }

    pub fn set_pending(&mut self, value: &Sequence<Any>) {
        self.inner.set("pending", value);
    }
}
/// The LockManager class.
/// [`LockManager`](https://developer.mozilla.org/en-US/docs/Web/API/LockManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockManager {
    inner: Any,
}
impl FromVal for LockManager {
    fn from_val(v: &Any) -> Self {
        LockManager {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LockManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LockManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LockManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LockManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LockManager> for Any {
    fn from(s: LockManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LockManager> for Any {
    fn from(s: &LockManager) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LockManager);

impl LockManager {
    /// The request method.
    /// [`LockManager.request`](https://developer.mozilla.org/en-US/docs/Web/API/LockManager/request)
    pub fn request(&self, name: &str, options: &LockOptions, callback: &Function) -> Promise<Any> {
        self.inner
            .call("request", &[name.into(), options.into(), callback.into()])
            .as_::<Promise<Any>>()
    }
}
impl LockManager {
    /// The query method.
    /// [`LockManager.query`](https://developer.mozilla.org/en-US/docs/Web/API/LockManager/query)
    pub fn query(&self) -> Promise<LockManagerSnapshot> {
        self.inner
            .call("query", &[])
            .as_::<Promise<LockManagerSnapshot>>()
    }
}
