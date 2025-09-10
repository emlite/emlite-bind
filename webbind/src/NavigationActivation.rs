use super::*;

/// The NavigationActivation class.
/// [`NavigationActivation`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationActivation {
    inner: Any,
}

impl FromVal for NavigationActivation {
    fn from_val(v: &Any) -> Self {
        NavigationActivation {
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

impl core::ops::Deref for NavigationActivation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationActivation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationActivation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationActivation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationActivation> for Any {
    fn from(s: NavigationActivation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationActivation> for Any {
    fn from(s: &NavigationActivation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NavigationActivation);

impl NavigationActivation {
    /// Getter of the `from` attribute.
    /// [`NavigationActivation.from`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation/from)
    pub fn from(&self) -> NavigationHistoryEntry {
        self.inner.get("from").as_::<NavigationHistoryEntry>()
    }
}
impl NavigationActivation {
    /// Getter of the `entry` attribute.
    /// [`NavigationActivation.entry`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation/entry)
    pub fn entry(&self) -> NavigationHistoryEntry {
        self.inner.get("entry").as_::<NavigationHistoryEntry>()
    }
}
impl NavigationActivation {
    /// Getter of the `navigationType` attribute.
    /// [`NavigationActivation.navigationType`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation/navigationType)
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }
}
