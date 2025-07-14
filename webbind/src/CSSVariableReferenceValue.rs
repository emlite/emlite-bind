use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSVariableReferenceValue {
    inner: emlite::Val,
}
impl FromVal for CSSVariableReferenceValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSVariableReferenceValue {
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
impl core::ops::Deref for CSSVariableReferenceValue {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSVariableReferenceValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSVariableReferenceValue {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSVariableReferenceValue {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSVariableReferenceValue> for emlite::Val {
    fn from(s: CSSVariableReferenceValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSVariableReferenceValue);

impl CSSVariableReferenceValue {
    pub fn new0(variable: jsbind::USVString) -> CSSVariableReferenceValue {
        Self {
            inner: emlite::Val::global("CSSVariableReferenceValue")
                .new(&[variable.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(
        variable: jsbind::USVString,
        fallback: CSSUnparsedValue,
    ) -> CSSVariableReferenceValue {
        Self {
            inner: emlite::Val::global("CSSVariableReferenceValue")
                .new(&[variable.into(), fallback.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl CSSVariableReferenceValue {
    pub fn variable(&self) -> jsbind::USVString {
        self.inner.get("variable").as_::<jsbind::USVString>()
    }

    pub fn set_variable(&mut self, value: jsbind::USVString) {
        self.inner.set("variable", value);
    }
}
impl CSSVariableReferenceValue {
    pub fn fallback(&self) -> CSSUnparsedValue {
        self.inner.get("fallback").as_::<CSSUnparsedValue>()
    }
}
