use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProgressEventInit {
    inner: Any,
}
impl FromVal for ProgressEventInit {
    fn from_val(v: &Any) -> Self {
        ProgressEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ProgressEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProgressEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ProgressEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ProgressEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ProgressEventInit> for Any {
    fn from(s: ProgressEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ProgressEventInit> for Any {
    fn from(s: &ProgressEventInit) -> Any {
        s.inner.clone()
    }
}

impl ProgressEventInit {
    pub fn length_computable(&self) -> bool {
        self.inner.get("lengthComputable").as_::<bool>()
    }

    pub fn set_length_computable(&mut self, value: bool) {
        self.inner.set("lengthComputable", value);
    }
}
impl ProgressEventInit {
    pub fn loaded(&self) -> f64 {
        self.inner.get("loaded").as_::<f64>()
    }

    pub fn set_loaded(&mut self, value: f64) {
        self.inner.set("loaded", value);
    }
}
impl ProgressEventInit {
    pub fn total(&self) -> f64 {
        self.inner.get("total").as_::<f64>()
    }

    pub fn set_total(&mut self, value: f64) {
        self.inner.set("total", value);
    }
}
