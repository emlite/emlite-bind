use super::*;

/// The SpeechGrammar class.
/// [`SpeechGrammar`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechGrammar {
    inner: Any,
}

impl FromVal for SpeechGrammar {
    fn from_val(v: &Any) -> Self {
        SpeechGrammar {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SpeechGrammar {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechGrammar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechGrammar {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechGrammar {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechGrammar> for Any {
    fn from(s: SpeechGrammar) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechGrammar> for Any {
    fn from(s: &SpeechGrammar) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechGrammar);

impl SpeechGrammar {
    /// Getter of the `src` attribute.
    /// [`SpeechGrammar.src`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    /// [`SpeechGrammar.src`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl SpeechGrammar {
    /// Getter of the `weight` attribute.
    /// [`SpeechGrammar.weight`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)
    pub fn weight(&self) -> f32 {
        self.inner.get("weight").as_::<f32>()
    }

    /// Setter of the `weight` attribute.
    /// [`SpeechGrammar.weight`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)
    pub fn set_weight(&mut self, value: f32) {
        self.inner.set("weight", value);
    }
}
