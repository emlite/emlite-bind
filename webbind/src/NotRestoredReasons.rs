use super::*;




/// The NotRestoredReasons class.
/// [`NotRestoredReasons`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasons)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotRestoredReasons {
    inner: Any,
}

impl FromVal for NotRestoredReasons {
    fn from_val(v: &Any) -> Self {
        NotRestoredReasons { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NotRestoredReasons {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NotRestoredReasons {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NotRestoredReasons {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NotRestoredReasons {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NotRestoredReasons> for Any {
    fn from(s: NotRestoredReasons) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NotRestoredReasons> for Any {
    fn from(s: &NotRestoredReasons) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NotRestoredReasons);


impl NotRestoredReasons {
    /// Getter of the `src` attribute.
    /// [`NotRestoredReasons.src`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasons/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

}
impl NotRestoredReasons {
    /// Getter of the `id` attribute.
    /// [`NotRestoredReasons.id`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasons/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

}
impl NotRestoredReasons {
    /// Getter of the `name` attribute.
    /// [`NotRestoredReasons.name`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasons/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

}
impl NotRestoredReasons {
    /// Getter of the `url` attribute.
    /// [`NotRestoredReasons.url`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasons/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

}
impl NotRestoredReasons {
    /// Getter of the `reasons` attribute.
    /// [`NotRestoredReasons.reasons`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasons/reasons)
    pub fn reasons(&self) -> TypedArray<NotRestoredReasonDetails> {
        self.inner.get("reasons").as_::<TypedArray<NotRestoredReasonDetails>>()
    }

}
impl NotRestoredReasons {
    /// Getter of the `children` attribute.
    /// [`NotRestoredReasons.children`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasons/children)
    pub fn children(&self) -> TypedArray<NotRestoredReasons> {
        self.inner.get("children").as_::<TypedArray<NotRestoredReasons>>()
    }

}
impl NotRestoredReasons {
    /// The toJSON method.
    /// [`NotRestoredReasons.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/NotRestoredReasons/toJSON)
    pub fn to_json(&self, ) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
