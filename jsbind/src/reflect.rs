use crate::{Any, Function, Sequence};

/// static Reflect struct, never constructed.
pub struct Reflect;

impl Reflect {
    /// Internal helper that returns the global `Reflect` object as an
    /// `emlite::Val`.
    #[inline]
    fn obj() -> emlite::Val {
        emlite::Val::global("Reflect")
    }

    /// [`Reflect.apply`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply)
    pub fn apply(target: &Function, this_arg: &Any, arguments: &Sequence<Any>) -> Any {
        Self::obj()
            .call(
                "apply",
                &[
                    target.clone().into(),
                    this_arg.clone(),
                    arguments.clone().into(),
                ],
            )
            .as_()
    }

    /// [`Reflect.construct`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/construct)
    pub fn construct(target: &Function, args: &Sequence<Any>, new_target: Option<&Any>) -> Any {
        let mut params: Vec<emlite::Val> = vec![target.clone().into(), args.clone().into()];
        if let Some(nt) = new_target {
            params.push(nt.clone());
        }
        Self::obj().call("construct", &params).as_()
    }

    /// [`Reflect.defineProperty`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/defineProperty)
    pub fn define_property(target: &Any, key: &Any, attributes: &Any) -> bool {
        Self::obj()
            .call(
                "defineProperty",
                &[target.clone(), key.clone(), attributes.clone()],
            )
            .as_()
    }

    /// [`Reflect.deleteProperty`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/deleteProperty)
    pub fn delete_property(target: &Any, key: &Any) -> bool {
        Self::obj()
            .call("deleteProperty", &[target.clone(), key.clone()])
            .as_()
    }

    /// [`Reflect.get`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get)
    pub fn get(target: &Any, key: &Any, receiver: Option<&Any>) -> Any {
        let mut params = vec![target.clone(), key.clone()];
        if let Some(r) = receiver {
            params.push(r.clone());
        }
        Self::obj().call("get", &params).as_()
    }

    /// [`Reflect.set`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set)
    pub fn set(target: &Any, key: &Any, value: &Any, receiver: Option<&Any>) -> bool {
        let mut params = vec![target.clone(), key.clone(), value.clone()];
        if let Some(r) = receiver {
            params.push(r.clone());
        }
        Self::obj().call("set", &params).as_()
    }

    /// [`Reflect.getOwnPropertyDescriptor`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getOwnPropertyDescriptor)
    pub fn get_own_property_descriptor(target: &Any, key: &Any) -> Any {
        Self::obj()
            .call("getOwnPropertyDescriptor", &[target.clone(), key.clone()])
            .as_()
    }

    /// [`Reflect.getPrototypeOf`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getPrototypeOf)
    pub fn get_prototype_of(target: &Any) -> Any {
        Self::obj().call("getPrototypeOf", &[target.clone()]).as_()
    }

    /// [`Reflect.has`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/has)
    pub fn has(target: &Any, key: &Any) -> bool {
        Self::obj()
            .call("has", &[target.clone(), key.clone()])
            .as_()
    }

    /// [`Reflect.isExtensible`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/isExtensible)
    pub fn is_extensible(target: &Any) -> bool {
        Self::obj().call("isExtensible", &[target.clone()]).as_()
    }

    /// [`Reflect.ownKeys`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys)
    pub fn own_keys(target: &Any) -> Sequence<Any> {
        Self::obj().call("ownKeys", &[target.clone()]).as_()
    }

    /// [`Reflect.preventExtensions`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/preventExtensions)
    pub fn prevent_extensions(target: &Any) -> bool {
        Self::obj()
            .call("preventExtensions", &[target.clone()])
            .as_()
    }

    /// [`Reflect.setPrototypeOf`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/setPrototypeOf)
    pub fn set_prototype_of(target: &Any, proto: &Any) -> bool {
        Self::obj()
            .call("setPrototypeOf", &[target.clone(), proto.clone()])
            .as_()
    }
}
