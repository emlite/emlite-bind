use super::*;

/// The SpeechGrammarList class.
/// [`SpeechGrammarList`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechGrammarList {
    inner: Any,
}
impl FromVal for SpeechGrammarList {
    fn from_val(v: &Any) -> Self {
        SpeechGrammarList {
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
impl core::ops::Deref for SpeechGrammarList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechGrammarList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SpeechGrammarList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SpeechGrammarList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SpeechGrammarList> for Any {
    fn from(s: SpeechGrammarList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SpeechGrammarList> for Any {
    fn from(s: &SpeechGrammarList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechGrammarList);

impl SpeechGrammarList {
    /// The `new SpeechGrammarList(..)` constructor, creating a new SpeechGrammarList instance
    pub fn new() -> SpeechGrammarList {
        Self {
            inner: Any::global("SpeechGrammarList").new(&[]).as_::<Any>(),
        }
    }
}
impl SpeechGrammarList {
    /// Getter of the `length` attribute.
    /// [`SpeechGrammarList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SpeechGrammarList {
    /// The item method.
    /// [`SpeechGrammarList.item`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/item)
    pub fn item(&self, index: u32) -> SpeechGrammar {
        self.inner
            .call("item", &[index.into()])
            .as_::<SpeechGrammar>()
    }
}
impl SpeechGrammarList {
    /// The addFromURI method.
    /// [`SpeechGrammarList.addFromURI`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)
    pub fn add_from_uri0(&self, src: &str) -> Undefined {
        self.inner
            .call("addFromURI", &[src.into()])
            .as_::<Undefined>()
    }
    /// The addFromURI method.
    /// [`SpeechGrammarList.addFromURI`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)
    pub fn add_from_uri1(&self, src: &str, weight: f32) -> Undefined {
        self.inner
            .call("addFromURI", &[src.into(), weight.into()])
            .as_::<Undefined>()
    }
}
impl SpeechGrammarList {
    /// The addFromString method.
    /// [`SpeechGrammarList.addFromString`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)
    pub fn add_from_string0(&self, string: &str) -> Undefined {
        self.inner
            .call("addFromString", &[string.into()])
            .as_::<Undefined>()
    }
    /// The addFromString method.
    /// [`SpeechGrammarList.addFromString`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)
    pub fn add_from_string1(&self, string: &str, weight: f32) -> Undefined {
        self.inner
            .call("addFromString", &[string.into(), weight.into()])
            .as_::<Undefined>()
    }
}
