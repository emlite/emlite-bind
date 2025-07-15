use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieListItem {
    inner: emlite::Val,
}
impl FromVal for CookieListItem {
    fn from_val(v: &emlite::Val) -> Self {
        CookieListItem { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieListItem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieListItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CookieListItem {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CookieListItem {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CookieListItem> for emlite::Val {
    fn from(s: CookieListItem) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CookieListItem {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: USVString) {
        self.inner.set("name", value);
    }

}
impl CookieListItem {
    pub fn value(&self) -> USVString {
        self.inner.get("value").as_::<USVString>()
    }

    pub fn set_value(&mut self, value: USVString) {
        self.inner.set("value", value);
    }

}
impl CookieListItem {
    pub fn domain(&self) -> USVString {
        self.inner.get("domain").as_::<USVString>()
    }

    pub fn set_domain(&mut self, value: USVString) {
        self.inner.set("domain", value);
    }

}
impl CookieListItem {
    pub fn path(&self) -> USVString {
        self.inner.get("path").as_::<USVString>()
    }

    pub fn set_path(&mut self, value: USVString) {
        self.inner.set("path", value);
    }

}
impl CookieListItem {
    pub fn expires(&self) -> Any {
        self.inner.get("expires").as_::<Any>()
    }

    pub fn set_expires(&mut self, value: Any) {
        self.inner.set("expires", value);
    }

}
impl CookieListItem {
    pub fn secure(&self) -> bool {
        self.inner.get("secure").as_::<bool>()
    }

    pub fn set_secure(&mut self, value: bool) {
        self.inner.set("secure", value);
    }

}
impl CookieListItem {
    pub fn same_site(&self) -> CookieSameSite {
        self.inner.get("sameSite").as_::<CookieSameSite>()
    }

    pub fn set_same_site(&mut self, value: CookieSameSite) {
        self.inner.set("sameSite", value);
    }

}
impl CookieListItem {
    pub fn partitioned(&self) -> bool {
        self.inner.get("partitioned").as_::<bool>()
    }

    pub fn set_partitioned(&mut self, value: bool) {
        self.inner.set("partitioned", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieStoreGetOptions {
    inner: emlite::Val,
}
impl FromVal for CookieStoreGetOptions {
    fn from_val(v: &emlite::Val) -> Self {
        CookieStoreGetOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieStoreGetOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieStoreGetOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CookieStoreGetOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CookieStoreGetOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CookieStoreGetOptions> for emlite::Val {
    fn from(s: CookieStoreGetOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CookieStoreGetOptions {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: USVString) {
        self.inner.set("name", value);
    }

}
impl CookieStoreGetOptions {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }

    pub fn set_url(&mut self, value: USVString) {
        self.inner.set("url", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieInit {
    inner: emlite::Val,
}
impl FromVal for CookieInit {
    fn from_val(v: &emlite::Val) -> Self {
        CookieInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CookieInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CookieInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CookieInit> for emlite::Val {
    fn from(s: CookieInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CookieInit {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: USVString) {
        self.inner.set("name", value);
    }

}
impl CookieInit {
    pub fn value(&self) -> USVString {
        self.inner.get("value").as_::<USVString>()
    }

    pub fn set_value(&mut self, value: USVString) {
        self.inner.set("value", value);
    }

}
impl CookieInit {
    pub fn expires(&self) -> Any {
        self.inner.get("expires").as_::<Any>()
    }

    pub fn set_expires(&mut self, value: Any) {
        self.inner.set("expires", value);
    }

}
impl CookieInit {
    pub fn domain(&self) -> USVString {
        self.inner.get("domain").as_::<USVString>()
    }

    pub fn set_domain(&mut self, value: USVString) {
        self.inner.set("domain", value);
    }

}
impl CookieInit {
    pub fn path(&self) -> USVString {
        self.inner.get("path").as_::<USVString>()
    }

    pub fn set_path(&mut self, value: USVString) {
        self.inner.set("path", value);
    }

}
impl CookieInit {
    pub fn same_site(&self) -> CookieSameSite {
        self.inner.get("sameSite").as_::<CookieSameSite>()
    }

    pub fn set_same_site(&mut self, value: CookieSameSite) {
        self.inner.set("sameSite", value);
    }

}
impl CookieInit {
    pub fn partitioned(&self) -> bool {
        self.inner.get("partitioned").as_::<bool>()
    }

    pub fn set_partitioned(&mut self, value: bool) {
        self.inner.set("partitioned", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieStoreDeleteOptions {
    inner: emlite::Val,
}
impl FromVal for CookieStoreDeleteOptions {
    fn from_val(v: &emlite::Val) -> Self {
        CookieStoreDeleteOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieStoreDeleteOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieStoreDeleteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CookieStoreDeleteOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CookieStoreDeleteOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CookieStoreDeleteOptions> for emlite::Val {
    fn from(s: CookieStoreDeleteOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CookieStoreDeleteOptions {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: USVString) {
        self.inner.set("name", value);
    }

}
impl CookieStoreDeleteOptions {
    pub fn domain(&self) -> USVString {
        self.inner.get("domain").as_::<USVString>()
    }

    pub fn set_domain(&mut self, value: USVString) {
        self.inner.set("domain", value);
    }

}
impl CookieStoreDeleteOptions {
    pub fn path(&self) -> USVString {
        self.inner.get("path").as_::<USVString>()
    }

    pub fn set_path(&mut self, value: USVString) {
        self.inner.set("path", value);
    }

}
impl CookieStoreDeleteOptions {
    pub fn partitioned(&self) -> bool {
        self.inner.get("partitioned").as_::<bool>()
    }

    pub fn set_partitioned(&mut self, value: bool) {
        self.inner.set("partitioned", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieStore {
    inner: EventTarget,
}
impl FromVal for CookieStore {
    fn from_val(v: &emlite::Val) -> Self {
        CookieStore { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieStore {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CookieStore {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CookieStore {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CookieStore> for emlite::Val {
    fn from(s: CookieStore) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CookieStore);


impl CookieStore {
    pub fn get0(&self, ) -> Promise {
        self.inner.call("get", &[]).as_::<Promise>()
    }

    pub fn get1(&self, options: CookieStoreGetOptions) -> Promise {
        self.inner.call("get", &[options.into(), ]).as_::<Promise>()
    }

}
impl CookieStore {
    pub fn get_all0(&self, ) -> Promise {
        self.inner.call("getAll", &[]).as_::<Promise>()
    }

    pub fn get_all1(&self, options: CookieStoreGetOptions) -> Promise {
        self.inner.call("getAll", &[options.into(), ]).as_::<Promise>()
    }

}
impl CookieStore {
    pub fn set(&self, options: CookieInit) -> Promise {
        self.inner.call("set", &[options.into(), ]).as_::<Promise>()
    }

}
impl CookieStore {
    pub fn delete(&self, options: CookieStoreDeleteOptions) -> Promise {
        self.inner.call("delete", &[options.into(), ]).as_::<Promise>()
    }

}
impl CookieStore {
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    pub fn set_onchange(&mut self, value: Any) {
        self.inner.set("onchange", value);
    }

}
