use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPictureOptions {
    inner: emlite::Val,
}
impl FromVal for DocumentPictureInPictureOptions {
    fn from_val(v: &emlite::Val) -> Self {
        DocumentPictureInPictureOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DocumentPictureInPictureOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentPictureInPictureOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DocumentPictureInPictureOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DocumentPictureInPictureOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DocumentPictureInPictureOptions> for emlite::Val {
    fn from(s: DocumentPictureInPictureOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DocumentPictureInPictureOptions {
    pub fn width(&self) -> u64 {
        self.inner.get("width").as_::<u64>()
    }

    pub fn set_width(&mut self, value: u64) {
        self.inner.set("width", value);
    }
}
impl DocumentPictureInPictureOptions {
    pub fn height(&self) -> u64 {
        self.inner.get("height").as_::<u64>()
    }

    pub fn set_height(&mut self, value: u64) {
        self.inner.set("height", value);
    }
}
impl DocumentPictureInPictureOptions {
    pub fn disallow_return_to_opener(&self) -> bool {
        self.inner.get("disallowReturnToOpener").as_::<bool>()
    }

    pub fn set_disallow_return_to_opener(&mut self, value: bool) {
        self.inner.set("disallowReturnToOpener", value);
    }
}
impl DocumentPictureInPictureOptions {
    pub fn prefer_initial_window_placement(&self) -> bool {
        self.inner.get("preferInitialWindowPlacement").as_::<bool>()
    }

    pub fn set_prefer_initial_window_placement(&mut self, value: bool) {
        self.inner.set("preferInitialWindowPlacement", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPicture {
    inner: EventTarget,
}
impl FromVal for DocumentPictureInPicture {
    fn from_val(v: &emlite::Val) -> Self {
        DocumentPictureInPicture {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DocumentPictureInPicture {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentPictureInPicture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DocumentPictureInPicture {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DocumentPictureInPicture {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DocumentPictureInPicture> for emlite::Val {
    fn from(s: DocumentPictureInPicture) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DocumentPictureInPicture);

impl DocumentPictureInPicture {
    pub fn request_window0(&self) -> Promise {
        self.inner.call("requestWindow", &[]).as_::<Promise>()
    }

    pub fn request_window1(&self, options: DocumentPictureInPictureOptions) -> Promise {
        self.inner
            .call("requestWindow", &[options.into()])
            .as_::<Promise>()
    }
}
impl DocumentPictureInPicture {
    pub fn window(&self) -> Window {
        self.inner.get("window").as_::<Window>()
    }
}
impl DocumentPictureInPicture {
    pub fn onenter(&self) -> Any {
        self.inner.get("onenter").as_::<Any>()
    }

    pub fn set_onenter(&mut self, value: Any) {
        self.inner.set("onenter", value);
    }
}
