use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SnapEventInit {
    inner: Any,
}
impl FromVal for SnapEventInit {
    fn from_val(v: &Any) -> Self {
        SnapEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SnapEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SnapEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SnapEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SnapEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SnapEventInit> for Any {
    fn from(s: SnapEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SnapEventInit> for Any {
    fn from(s: &SnapEventInit) -> Any {
        s.inner.clone()
    }
}

impl SnapEventInit {
    pub fn snap_target_block(&self) -> Node {
        self.inner.get("snapTargetBlock").as_::<Node>()
    }

    pub fn set_snap_target_block(&mut self, value: &Node) {
        self.inner.set("snapTargetBlock", value);
    }
}
impl SnapEventInit {
    pub fn snap_target_inline(&self) -> Node {
        self.inner.get("snapTargetInline").as_::<Node>()
    }

    pub fn set_snap_target_inline(&mut self, value: &Node) {
        self.inner.set("snapTargetInline", value);
    }
}
