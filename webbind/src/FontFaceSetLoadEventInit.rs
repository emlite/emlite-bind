use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFaceSetLoadEventInit {
    inner: Any,
}
impl FromVal for FontFaceSetLoadEventInit {
    fn from_val(v: &Any) -> Self {
        FontFaceSetLoadEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FontFaceSetLoadEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFaceSetLoadEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FontFaceSetLoadEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FontFaceSetLoadEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FontFaceSetLoadEventInit> for Any {
    fn from(s: FontFaceSetLoadEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FontFaceSetLoadEventInit> for Any {
    fn from(s: &FontFaceSetLoadEventInit) -> Any {
        s.inner.clone()
    }
}

impl FontFaceSetLoadEventInit {
    pub fn fontfaces(&self) -> TypedArray<FontFace> {
        self.inner.get("fontfaces").as_::<TypedArray<FontFace>>()
    }

    pub fn set_fontfaces(&mut self, value: &TypedArray<FontFace>) {
        self.inner.set("fontfaces", value);
    }
}
