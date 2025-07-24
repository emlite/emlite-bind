use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieListItem {
    inner: Any,
}
impl FromVal for CookieListItem {
    fn from_val(v: &Any) -> Self {
        CookieListItem { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieListItem {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieListItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CookieListItem {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CookieListItem {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CookieListItem> for Any {
    fn from(s: CookieListItem) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CookieListItem> for Any {
    fn from(s: &CookieListItem) -> Any {
        s.inner.clone()
    }
}

impl CookieListItem {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: &USVString) {
        self.inner.set("name", value);
    }
}
impl CookieListItem {
    pub fn value(&self) -> USVString {
        self.inner.get("value").as_::<USVString>()
    }

    pub fn set_value(&mut self, value: &USVString) {
        self.inner.set("value", value);
    }
}
impl CookieListItem {
    pub fn domain(&self) -> USVString {
        self.inner.get("domain").as_::<USVString>()
    }

    pub fn set_domain(&mut self, value: &USVString) {
        self.inner.set("domain", value);
    }
}
impl CookieListItem {
    pub fn path(&self) -> USVString {
        self.inner.get("path").as_::<USVString>()
    }

    pub fn set_path(&mut self, value: &USVString) {
        self.inner.set("path", value);
    }
}
impl CookieListItem {
    pub fn expires(&self) -> Any {
        self.inner.get("expires").as_::<Any>()
    }

    pub fn set_expires(&mut self, value: &Any) {
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

    pub fn set_same_site(&mut self, value: &CookieSameSite) {
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
    inner: Any,
}
impl FromVal for CookieStoreGetOptions {
    fn from_val(v: &Any) -> Self {
        CookieStoreGetOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieStoreGetOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieStoreGetOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CookieStoreGetOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CookieStoreGetOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CookieStoreGetOptions> for Any {
    fn from(s: CookieStoreGetOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CookieStoreGetOptions> for Any {
    fn from(s: &CookieStoreGetOptions) -> Any {
        s.inner.clone()
    }
}

impl CookieStoreGetOptions {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: &USVString) {
        self.inner.set("name", value);
    }
}
impl CookieStoreGetOptions {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }

    pub fn set_url(&mut self, value: &USVString) {
        self.inner.set("url", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieInit {
    inner: Any,
}
impl FromVal for CookieInit {
    fn from_val(v: &Any) -> Self {
        CookieInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CookieInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CookieInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CookieInit> for Any {
    fn from(s: CookieInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CookieInit> for Any {
    fn from(s: &CookieInit) -> Any {
        s.inner.clone()
    }
}

impl CookieInit {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: &USVString) {
        self.inner.set("name", value);
    }
}
impl CookieInit {
    pub fn value(&self) -> USVString {
        self.inner.get("value").as_::<USVString>()
    }

    pub fn set_value(&mut self, value: &USVString) {
        self.inner.set("value", value);
    }
}
impl CookieInit {
    pub fn expires(&self) -> Any {
        self.inner.get("expires").as_::<Any>()
    }

    pub fn set_expires(&mut self, value: &Any) {
        self.inner.set("expires", value);
    }
}
impl CookieInit {
    pub fn domain(&self) -> USVString {
        self.inner.get("domain").as_::<USVString>()
    }

    pub fn set_domain(&mut self, value: &USVString) {
        self.inner.set("domain", value);
    }
}
impl CookieInit {
    pub fn path(&self) -> USVString {
        self.inner.get("path").as_::<USVString>()
    }

    pub fn set_path(&mut self, value: &USVString) {
        self.inner.set("path", value);
    }
}
impl CookieInit {
    pub fn same_site(&self) -> CookieSameSite {
        self.inner.get("sameSite").as_::<CookieSameSite>()
    }

    pub fn set_same_site(&mut self, value: &CookieSameSite) {
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
    inner: Any,
}
impl FromVal for CookieStoreDeleteOptions {
    fn from_val(v: &Any) -> Self {
        CookieStoreDeleteOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieStoreDeleteOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieStoreDeleteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CookieStoreDeleteOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CookieStoreDeleteOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CookieStoreDeleteOptions> for Any {
    fn from(s: CookieStoreDeleteOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CookieStoreDeleteOptions> for Any {
    fn from(s: &CookieStoreDeleteOptions) -> Any {
        s.inner.clone()
    }
}

impl CookieStoreDeleteOptions {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }

    pub fn set_name(&mut self, value: &USVString) {
        self.inner.set("name", value);
    }
}
impl CookieStoreDeleteOptions {
    pub fn domain(&self) -> USVString {
        self.inner.get("domain").as_::<USVString>()
    }

    pub fn set_domain(&mut self, value: &USVString) {
        self.inner.set("domain", value);
    }
}
impl CookieStoreDeleteOptions {
    pub fn path(&self) -> USVString {
        self.inner.get("path").as_::<USVString>()
    }

    pub fn set_path(&mut self, value: &USVString) {
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
/// The CookieStore class.
/// [`CookieStore`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieStore {
    inner: EventTarget,
}
impl FromVal for CookieStore {
    fn from_val(v: &Any) -> Self {
        CookieStore {
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
impl AsRef<Any> for CookieStore {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CookieStore {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CookieStore> for Any {
    fn from(s: CookieStore) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CookieStore> for Any {
    fn from(s: &CookieStore) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CookieStore);

impl CookieStore {
    /// The get method.
    /// [`CookieStore.get`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get)
    pub fn get0(&self) -> Promise<CookieListItem> {
        self.inner.call("get", &[]).as_::<Promise<CookieListItem>>()
    }
    /// The get method.
    /// [`CookieStore.get`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get)
    pub fn get1(&self, options: &CookieStoreGetOptions) -> Promise<CookieListItem> {
        self.inner
            .call("get", &[options.into()])
            .as_::<Promise<CookieListItem>>()
    }
}
impl CookieStore {
    /// The getAll method.
    /// [`CookieStore.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll)
    pub fn get_all0(&self) -> Promise<Any> {
        self.inner.call("getAll", &[]).as_::<Promise<Any>>()
    }
    /// The getAll method.
    /// [`CookieStore.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll)
    pub fn get_all1(&self, options: &CookieStoreGetOptions) -> Promise<Any> {
        self.inner
            .call("getAll", &[options.into()])
            .as_::<Promise<Any>>()
    }
}
impl CookieStore {
    /// The set method.
    /// [`CookieStore.set`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/set)
    pub fn set(&self, options: &CookieInit) -> Promise<Undefined> {
        self.inner
            .call("set", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl CookieStore {
    /// The delete method.
    /// [`CookieStore.delete`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/delete)
    pub fn delete(&self, options: &CookieStoreDeleteOptions) -> Promise<Undefined> {
        self.inner
            .call("delete", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl CookieStore {
    /// Getter of the `onchange` attribute.
    /// [`CookieStore.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`CookieStore.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
