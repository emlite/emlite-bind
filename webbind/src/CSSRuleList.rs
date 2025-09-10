use super::*;

/// The CSSRuleList class.
/// [`CSSRuleList`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSRuleList {
    inner: Any,
}

impl FromVal for CSSRuleList {
    fn from_val(v: &Any) -> Self {
        CSSRuleList {
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

impl core::ops::Deref for CSSRuleList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSRuleList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSRuleList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSRuleList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSRuleList> for Any {
    fn from(s: CSSRuleList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSRuleList> for Any {
    fn from(s: &CSSRuleList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSRuleList);

impl CSSRuleList {
    /// The item method.
    /// [`CSSRuleList.item`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/item)
    pub fn item(&self, index: u32) -> CSSRule {
        self.inner.call("item", &[index.into()]).as_::<CSSRule>()
    }
}
impl CSSRuleList {
    /// Getter of the `length` attribute.
    /// [`CSSRuleList.length`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
