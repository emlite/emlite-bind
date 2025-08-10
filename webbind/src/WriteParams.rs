use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WriteParams {
    inner: Any,
}
impl FromVal for WriteParams {
    fn from_val(v: &Any) -> Self {
        WriteParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WriteParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WriteParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WriteParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WriteParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WriteParams> for Any {
    fn from(s: WriteParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WriteParams> for Any {
    fn from(s: &WriteParams) -> Any {
        s.inner.clone()
    }
}

impl WriteParams {
    pub fn type_(&self) -> WriteCommandType {
        self.inner.get("type").as_::<WriteCommandType>()
    }

    pub fn set_type_(&mut self, value: &WriteCommandType) {
        self.inner.set("type", value);
    }
}
impl WriteParams {
    pub fn size(&self) -> u64 {
        self.inner.get("size").as_::<u64>()
    }

    pub fn set_size(&mut self, value: u64) {
        self.inner.set("size", value);
    }
}
impl WriteParams {
    pub fn position(&self) -> u64 {
        self.inner.get("position").as_::<u64>()
    }

    pub fn set_position(&mut self, value: u64) {
        self.inner.set("position", value);
    }
}
impl WriteParams {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
