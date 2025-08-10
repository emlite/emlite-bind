use super::*;

/// The MLTriangularOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTriangularOptions {
    inner: Any,
}

impl FromVal for MLTriangularOptions {
    fn from_val(v: &Any) -> Self {
        MLTriangularOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLTriangularOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLTriangularOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLTriangularOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLTriangularOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLTriangularOptions> for Any {
    fn from(s: MLTriangularOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLTriangularOptions> for Any {
    fn from(s: &MLTriangularOptions) -> Any {
        s.inner.clone()
    }
}

impl MLTriangularOptions {
    /// Getter of the `upper` attribute.
    pub fn upper(&self) -> bool {
        self.inner.get("upper").as_::<bool>()
    }

    /// Setter of the `upper` attribute.
    pub fn set_upper(&mut self, value: bool) {
        self.inner.set("upper", value);
    }
}
impl MLTriangularOptions {
    /// Getter of the `diagonal` attribute.
    pub fn diagonal(&self) -> i32 {
        self.inner.get("diagonal").as_::<i32>()
    }

    /// Setter of the `diagonal` attribute.
    pub fn set_diagonal(&mut self, value: i32) {
        self.inner.set("diagonal", value);
    }
}
