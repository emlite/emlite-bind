use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockOptions {
    inner: emlite::Val,
}
impl FromVal for LockOptions {
    fn from_val(v: &emlite::Val) -> Self {
        LockOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LockOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LockOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LockOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LockOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LockOptions> for emlite::Val {
    fn from(s: LockOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LockOptions {
    pub fn mode(&self) -> LockMode {
        self.inner.get("mode").as_::<LockMode>()
    }

    pub fn set_mode(&mut self, value: LockMode) {
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

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockManagerSnapshot {
    inner: emlite::Val,
}
impl FromVal for LockManagerSnapshot {
    fn from_val(v: &emlite::Val) -> Self {
        LockManagerSnapshot { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LockManagerSnapshot {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LockManagerSnapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LockManagerSnapshot {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LockManagerSnapshot {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LockManagerSnapshot> for emlite::Val {
    fn from(s: LockManagerSnapshot) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LockManagerSnapshot {
    pub fn held(&self) -> Sequence<Any> {
        self.inner.get("held").as_::<Sequence<Any>>()
    }

    pub fn set_held(&mut self, value: Sequence<Any>) {
        self.inner.set("held", value);
    }
}
impl LockManagerSnapshot {
    pub fn pending(&self) -> Sequence<Any> {
        self.inner.get("pending").as_::<Sequence<Any>>()
    }

    pub fn set_pending(&mut self, value: Sequence<Any>) {
        self.inner.set("pending", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockManager {
    inner: emlite::Val,
}
impl FromVal for LockManager {
    fn from_val(v: &emlite::Val) -> Self {
        LockManager {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LockManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LockManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LockManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LockManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LockManager> for emlite::Val {
    fn from(s: LockManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(LockManager);

impl LockManager {
    pub fn request(&self, name: DOMString, options: LockOptions, callback: Function) -> Promise {
        self.inner
            .call("request", &[name.into(), options.into(), callback.into()])
            .as_::<Promise>()
    }
}
impl LockManager {
    pub fn query(&self) -> Promise {
        self.inner.call("query", &[]).as_::<Promise>()
    }
}
