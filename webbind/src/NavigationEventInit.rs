use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationEventInit {
    inner: Any,
}
impl FromVal for NavigationEventInit {
    fn from_val(v: &Any) -> Self {
        NavigationEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationEventInit> for Any {
    fn from(s: NavigationEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationEventInit> for Any {
    fn from(s: &NavigationEventInit) -> Any {
        s.inner.clone()
    }
}

impl NavigationEventInit {
    pub fn dir(&self) -> SpatialNavigationDirection {
        self.inner.get("dir").as_::<SpatialNavigationDirection>()
    }

    pub fn set_dir(&mut self, value: &SpatialNavigationDirection) {
        self.inner.set("dir", value);
    }
}
impl NavigationEventInit {
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }

    pub fn set_related_target(&mut self, value: &EventTarget) {
        self.inner.set("relatedTarget", value);
    }
}
