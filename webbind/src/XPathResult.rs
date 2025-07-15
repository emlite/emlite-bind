use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XPathResult {
    inner: emlite::Val,
}
impl FromVal for XPathResult {
    fn from_val(v: &emlite::Val) -> Self {
        XPathResult {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XPathResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XPathResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XPathResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XPathResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XPathResult> for emlite::Val {
    fn from(s: XPathResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XPathResult> for emlite::Val {
    fn from(s: &XPathResult) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XPathResult);

impl XPathResult {
    pub fn result_type(&self) -> u16 {
        self.inner.get("resultType").as_::<u16>()
    }
}
impl XPathResult {
    pub fn number_value(&self) -> f64 {
        self.inner.get("numberValue").as_::<f64>()
    }
}
impl XPathResult {
    pub fn string_value(&self) -> String {
        self.inner.get("stringValue").as_::<String>()
    }
}
impl XPathResult {
    pub fn boolean_value(&self) -> bool {
        self.inner.get("booleanValue").as_::<bool>()
    }
}
impl XPathResult {
    pub fn single_node_value(&self) -> Node {
        self.inner.get("singleNodeValue").as_::<Node>()
    }
}
impl XPathResult {
    pub fn invalid_iterator_state(&self) -> bool {
        self.inner.get("invalidIteratorState").as_::<bool>()
    }
}
impl XPathResult {
    pub fn snapshot_length(&self) -> u32 {
        self.inner.get("snapshotLength").as_::<u32>()
    }
}
impl XPathResult {
    pub fn iterate_next(&self) -> Node {
        self.inner.call("iterateNext", &[]).as_::<Node>()
    }
}
impl XPathResult {
    pub fn snapshot_item(&self, index: u32) -> Node {
        self.inner
            .call("snapshotItem", &[index.into()])
            .as_::<Node>()
    }
}
