use super::*;

/// The SpeechSynthesis class.
/// [`SpeechSynthesis`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechSynthesis {
    inner: EventTarget,
}
impl FromVal for SpeechSynthesis {
    fn from_val(v: &Any) -> Self {
        SpeechSynthesis {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpeechSynthesis {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechSynthesis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SpeechSynthesis {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SpeechSynthesis {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SpeechSynthesis> for Any {
    fn from(s: SpeechSynthesis) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SpeechSynthesis> for Any {
    fn from(s: &SpeechSynthesis) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechSynthesis);

impl SpeechSynthesis {
    /// Getter of the `pending` attribute.
    /// [`SpeechSynthesis.pending`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pending)
    pub fn pending(&self) -> bool {
        self.inner.get("pending").as_::<bool>()
    }
}
impl SpeechSynthesis {
    /// Getter of the `speaking` attribute.
    /// [`SpeechSynthesis.speaking`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speaking)
    pub fn speaking(&self) -> bool {
        self.inner.get("speaking").as_::<bool>()
    }
}
impl SpeechSynthesis {
    /// Getter of the `paused` attribute.
    /// [`SpeechSynthesis.paused`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/paused)
    pub fn paused(&self) -> bool {
        self.inner.get("paused").as_::<bool>()
    }
}
impl SpeechSynthesis {
    /// Getter of the `onvoiceschanged` attribute.
    /// [`SpeechSynthesis.onvoiceschanged`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)
    pub fn onvoiceschanged(&self) -> Any {
        self.inner.get("onvoiceschanged").as_::<Any>()
    }

    /// Setter of the `onvoiceschanged` attribute.
    /// [`SpeechSynthesis.onvoiceschanged`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)
    pub fn set_onvoiceschanged(&mut self, value: &Any) {
        self.inner.set("onvoiceschanged", value);
    }
}
impl SpeechSynthesis {
    /// The speak method.
    /// [`SpeechSynthesis.speak`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speak)
    pub fn speak(&self, utterance: &SpeechSynthesisUtterance) -> Undefined {
        self.inner
            .call("speak", &[utterance.into()])
            .as_::<Undefined>()
    }
}
impl SpeechSynthesis {
    /// The cancel method.
    /// [`SpeechSynthesis.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/cancel)
    pub fn cancel(&self) -> Undefined {
        self.inner.call("cancel", &[]).as_::<Undefined>()
    }
}
impl SpeechSynthesis {
    /// The pause method.
    /// [`SpeechSynthesis.pause`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pause)
    pub fn pause(&self) -> Undefined {
        self.inner.call("pause", &[]).as_::<Undefined>()
    }
}
impl SpeechSynthesis {
    /// The resume method.
    /// [`SpeechSynthesis.resume`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/resume)
    pub fn resume(&self) -> Undefined {
        self.inner.call("resume", &[]).as_::<Undefined>()
    }
}
impl SpeechSynthesis {
    /// The getVoices method.
    /// [`SpeechSynthesis.getVoices`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/getVoices)
    pub fn get_voices(&self) -> Sequence<SpeechSynthesisVoice> {
        self.inner
            .call("getVoices", &[])
            .as_::<Sequence<SpeechSynthesisVoice>>()
    }
}
