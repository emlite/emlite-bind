use super::*;

/// The MLClampOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLClampOptions {
    inner: Any,
}

impl FromVal for MLClampOptions {
    fn from_val(v: &Any) -> Self {
        MLClampOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLClampOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLClampOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLClampOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLClampOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLClampOptions> for Any {
    fn from(s: MLClampOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLClampOptions> for Any {
    fn from(s: &MLClampOptions) -> Any {
        s.inner.clone()
    }
}

impl MLClampOptions {
    /// Getter of the `minValue` attribute.
    pub fn min_value(&self) -> Any {
        self.inner.get("minValue").as_::<Any>()
    }

    /// Setter of the `minValue` attribute.
    pub fn set_min_value(&mut self, value: &Any) {
        self.inner.set("minValue", value);
    }
}
impl MLClampOptions {
    /// Getter of the `maxValue` attribute.
    pub fn max_value(&self) -> Any {
        self.inner.get("maxValue").as_::<Any>()
    }

    /// Setter of the `maxValue` attribute.
    pub fn set_max_value(&mut self, value: &Any) {
        self.inner.set("maxValue", value);
    }
}
