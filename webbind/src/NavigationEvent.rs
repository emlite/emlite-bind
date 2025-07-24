use super::*;

/// The NavigationEvent class.
/// [`NavigationEvent`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationEvent {
    inner: UIEvent,
}
impl FromVal for NavigationEvent {
    fn from_val(v: &Any) -> Self {
        NavigationEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationEvent> for Any {
    fn from(s: NavigationEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationEvent> for Any {
    fn from(s: &NavigationEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigationEvent);

impl NavigationEvent {
    /// The `new NavigationEvent(..)` constructor, creating a new NavigationEvent instance
    pub fn new0(type_: &DOMString) -> NavigationEvent {
        Self {
            inner: Any::global("NavigationEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    /// The `new NavigationEvent(..)` constructor, creating a new NavigationEvent instance
    pub fn new1(type_: &DOMString, event_init_dict: &Any) -> NavigationEvent {
        Self {
            inner: Any::global("NavigationEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl NavigationEvent {
    /// Getter of the `dir` attribute.
    /// [`NavigationEvent.dir`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationEvent/dir)
    pub fn dir(&self) -> SpatialNavigationDirection {
        self.inner.get("dir").as_::<SpatialNavigationDirection>()
    }
}
impl NavigationEvent {
    /// Getter of the `relatedTarget` attribute.
    /// [`NavigationEvent.relatedTarget`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationEvent/relatedTarget)
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }
}
