use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CharacterBoundsUpdateEventInit {
    inner: Any,
}
impl FromVal for CharacterBoundsUpdateEventInit {
    fn from_val(v: &Any) -> Self {
        CharacterBoundsUpdateEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CharacterBoundsUpdateEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CharacterBoundsUpdateEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CharacterBoundsUpdateEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CharacterBoundsUpdateEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CharacterBoundsUpdateEventInit> for Any {
    fn from(s: CharacterBoundsUpdateEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CharacterBoundsUpdateEventInit> for Any {
    fn from(s: &CharacterBoundsUpdateEventInit) -> Any {
        s.inner.clone()
    }
}

impl CharacterBoundsUpdateEventInit {
    pub fn range_start(&self) -> u32 {
        self.inner.get("rangeStart").as_::<u32>()
    }

    pub fn set_range_start(&mut self, value: u32) {
        self.inner.set("rangeStart", value);
    }
}
impl CharacterBoundsUpdateEventInit {
    pub fn range_end(&self) -> u32 {
        self.inner.get("rangeEnd").as_::<u32>()
    }

    pub fn set_range_end(&mut self, value: u32) {
        self.inner.set("rangeEnd", value);
    }
}
