use super::*;

#[derive(Clone, Debug)]
pub struct NotRestoredReasons {
    inner: emlite::Val,
}
impl FromVal for NotRestoredReasons {
    fn from_val(v: &emlite::Val) -> Self {
        NotRestoredReasons {
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
impl std::ops::Deref for NotRestoredReasons {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NotRestoredReasons {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NotRestoredReasons> for emlite::Val {
    fn from(s: NotRestoredReasons) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NotRestoredReasons {
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }
}
impl NotRestoredReasons {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl NotRestoredReasons {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl NotRestoredReasons {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl NotRestoredReasons {
    pub fn reasons(&self) -> jsbind::FrozenArray<NotRestoredReasonDetails> {
        self.inner
            .get("reasons")
            .as_::<jsbind::FrozenArray<NotRestoredReasonDetails>>()
    }
}
impl NotRestoredReasons {
    pub fn children(&self) -> jsbind::FrozenArray<NotRestoredReasons> {
        self.inner
            .get("children")
            .as_::<jsbind::FrozenArray<NotRestoredReasons>>()
    }
}
impl NotRestoredReasons {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
