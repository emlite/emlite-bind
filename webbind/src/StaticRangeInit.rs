use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StaticRangeInit {
    inner: Any,
}
impl FromVal for StaticRangeInit {
    fn from_val(v: &Any) -> Self {
        StaticRangeInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StaticRangeInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StaticRangeInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StaticRangeInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StaticRangeInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StaticRangeInit> for Any {
    fn from(s: StaticRangeInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StaticRangeInit> for Any {
    fn from(s: &StaticRangeInit) -> Any {
        s.inner.clone()
    }
}

impl StaticRangeInit {
    pub fn start_container(&self) -> Node {
        self.inner.get("startContainer").as_::<Node>()
    }

    pub fn set_start_container(&mut self, value: &Node) {
        self.inner.set("startContainer", value);
    }
}
impl StaticRangeInit {
    pub fn start_offset(&self) -> u32 {
        self.inner.get("startOffset").as_::<u32>()
    }

    pub fn set_start_offset(&mut self, value: u32) {
        self.inner.set("startOffset", value);
    }
}
impl StaticRangeInit {
    pub fn end_container(&self) -> Node {
        self.inner.get("endContainer").as_::<Node>()
    }

    pub fn set_end_container(&mut self, value: &Node) {
        self.inner.set("endContainer", value);
    }
}
impl StaticRangeInit {
    pub fn end_offset(&self) -> u32 {
        self.inner.get("endOffset").as_::<u32>()
    }

    pub fn set_end_offset(&mut self, value: u32) {
        self.inner.set("endOffset", value);
    }
}
