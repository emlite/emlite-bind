use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for Client {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Client {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Client {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Client> for emlite::Val {
    fn from(s: Client) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Client> for emlite::Val {
    fn from(s: &Client) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Client);

impl Client {
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }
}
impl Client {
    pub fn frame_type(&self) -> FrameType {
        self.inner.get("frameType").as_::<FrameType>()
    }
}
impl Client {
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl Client {
    pub fn type_(&self) -> ClientType {
        self.inner.get("type").as_::<ClientType>()
    }
}
impl Client {
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }

    pub fn post_message1(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl Client {
    pub fn lifecycle_state(&self) -> ClientLifecycleState {
        self.inner
            .get("lifecycleState")
            .as_::<ClientLifecycleState>()
    }
}
