use super::*;

#[derive(Clone, Debug)]
pub struct Client {
    inner: emlite::Val,
}
impl FromVal for Client {
    fn from_val(v: &emlite::Val) -> Self {
        Client {
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
impl std::ops::Deref for Client {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Client> for emlite::Val {
    fn from(s: Client) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Client {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl Client {
    pub fn frame_type(&self) -> FrameType {
        self.inner.get("frameType").as_::<FrameType>()
    }
}
impl Client {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl Client {
    pub fn type_(&self) -> ClientType {
        self.inner.get("type").as_::<ClientType>()
    }
}
impl Client {
    pub fn post_message0(&self, message: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn post_message1(
        &self,
        message: jsbind::Any,
        options: StructuredSerializeOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Client {
    pub fn lifecycle_state(&self) -> ClientLifecycleState {
        self.inner
            .get("lifecycleState")
            .as_::<ClientLifecycleState>()
    }
}
