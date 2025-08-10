use super::*;

/// The NavigationTransition class.
/// [`NavigationTransition`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationTransition {
    inner: Any,
}

impl FromVal for NavigationTransition {
    fn from_val(v: &Any) -> Self {
        NavigationTransition {
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

impl core::ops::Deref for NavigationTransition {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationTransition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationTransition {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationTransition {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationTransition> for Any {
    fn from(s: NavigationTransition) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationTransition> for Any {
    fn from(s: &NavigationTransition) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NavigationTransition);

impl NavigationTransition {
    /// Getter of the `navigationType` attribute.
    /// [`NavigationTransition.navigationType`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition/navigationType)
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }
}
impl NavigationTransition {
    /// Getter of the `from` attribute.
    /// [`NavigationTransition.from`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition/from)
    pub fn from(&self) -> NavigationHistoryEntry {
        self.inner.get("from").as_::<NavigationHistoryEntry>()
    }
}
impl NavigationTransition {
    /// Getter of the `finished` attribute.
    /// [`NavigationTransition.finished`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationTransition/finished)
    pub fn finished(&self) -> Promise<Undefined> {
        self.inner.get("finished").as_::<Promise<Undefined>>()
    }
}
