use super::*;

/// The SpeechRecognitionPhraseList class.
/// [`SpeechRecognitionPhraseList`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionPhraseList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionPhraseList {
    inner: Any,
}

impl FromVal for SpeechRecognitionPhraseList {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionPhraseList {
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

impl core::ops::Deref for SpeechRecognitionPhraseList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechRecognitionPhraseList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechRecognitionPhraseList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionPhraseList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechRecognitionPhraseList> for Any {
    fn from(s: SpeechRecognitionPhraseList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionPhraseList> for Any {
    fn from(s: &SpeechRecognitionPhraseList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechRecognitionPhraseList);

impl SpeechRecognitionPhraseList {
    /// The `new SpeechRecognitionPhraseList(..)` constructor, creating a new SpeechRecognitionPhraseList instance
    pub fn new(phrases: &TypedArray<SpeechRecognitionPhrase>) -> SpeechRecognitionPhraseList {
        Self {
            inner: Any::global("SpeechRecognitionPhraseList")
                .new(&[phrases.into()])
                .as_::<Any>(),
        }
    }
}
impl SpeechRecognitionPhraseList {
    /// Getter of the `length` attribute.
    /// [`SpeechRecognitionPhraseList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionPhraseList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SpeechRecognitionPhraseList {
    /// The item method.
    /// [`SpeechRecognitionPhraseList.item`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionPhraseList/item)
    pub fn item(&self, index: u32) -> SpeechRecognitionPhrase {
        self.inner
            .call("item", &[index.into()])
            .as_::<SpeechRecognitionPhrase>()
    }
}
impl SpeechRecognitionPhraseList {
    /// The addItem method.
    /// [`SpeechRecognitionPhraseList.addItem`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionPhraseList/addItem)
    pub fn add_item(&self, item: &SpeechRecognitionPhrase) -> Undefined {
        self.inner
            .call("addItem", &[item.into()])
            .as_::<Undefined>()
    }
}
impl SpeechRecognitionPhraseList {
    /// The removeItem method.
    /// [`SpeechRecognitionPhraseList.removeItem`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionPhraseList/removeItem)
    pub fn remove_item(&self, index: u32) -> Undefined {
        self.inner
            .call("removeItem", &[index.into()])
            .as_::<Undefined>()
    }
}
