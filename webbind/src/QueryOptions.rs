use super::*;

/// The QueryOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct QueryOptions {
    inner: Any,
}

impl FromVal for QueryOptions {
    fn from_val(v: &Any) -> Self {
        QueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for QueryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for QueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for QueryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for QueryOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<QueryOptions> for Any {
    fn from(s: QueryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&QueryOptions> for Any {
    fn from(s: &QueryOptions) -> Any {
        s.inner.clone()
    }
}

impl QueryOptions {
    /// Getter of the `postscriptNames` attribute.
    pub fn postscript_names(&self) -> TypedArray<JsString> {
        self.inner
            .get("postscriptNames")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `postscriptNames` attribute.
    pub fn set_postscript_names(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("postscriptNames", value);
    }
}
