use crate::{Any, Function, Sequence};

/// Zero-sized namespace type; all methods are `fn` on the impl.
pub struct Reflect;

impl Reflect {
    #[inline]
    fn obj() -> emlite::Val {
        // adjust if your helper is named differently
        emlite::Val::global("Reflect")
    }

    /* ───────── 1. Function application / construction ───────── */

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

    pub fn construct(target: &Function, args: &Sequence<Any>, new_target: Option<&Any>) -> Any {
        let mut params: Vec<emlite::Val> = vec![target.clone().into(), args.clone().into()];
        if let Some(nt) = new_target {
            params.push(nt.clone());
        }
        Self::obj().call("construct", &params).as_()
    }

    /* ───────── 2. Property definition / deletion ───────── */

    pub fn define_property(target: &Any, key: &Any, attributes: &Any) -> bool {
        Self::obj()
            .call(
                "defineProperty",
                &[target.clone(), key.clone(), attributes.clone()],
            )
            .as_()
    }

    pub fn delete_property(target: &Any, key: &Any) -> bool {
        Self::obj()
            .call("deleteProperty", &[target.clone(), key.clone()])
            .as_()
    }

    /* ───────── 3. Getters / setters ───────── */

    pub fn get(target: &Any, key: &Any, receiver: Option<&Any>) -> Any {
        let mut params = vec![target.clone(), key.clone()];
        if let Some(r) = receiver {
            params.push(r.clone());
        }
        Self::obj().call("get", &params).as_()
    }

    pub fn set(target: &Any, key: &Any, value: &Any, receiver: Option<&Any>) -> bool {
        let mut params = vec![target.clone(), key.clone(), value.clone()];
        if let Some(r) = receiver {
            params.push(r.clone());
        }
        Self::obj().call("set", &params).as_()
    }

    /* ───────── 4. Introspection helpers ───────── */

    pub fn get_own_property_descriptor(target: &Any, key: &Any) -> Any {
        Self::obj()
            .call("getOwnPropertyDescriptor", &[target.clone(), key.clone()])
            .as_()
    }

    pub fn get_prototype_of(target: &Any) -> Any {
        Self::obj().call("getPrototypeOf", &[target.clone()]).as_()
    }

    pub fn has(target: &Any, key: &Any) -> bool {
        Self::obj()
            .call("has", &[target.clone(), key.clone()])
            .as_()
    }

    pub fn is_extensible(target: &Any) -> bool {
        Self::obj().call("isExtensible", &[target.clone()]).as_()
    }

    pub fn own_keys(target: &Any) -> Sequence<Any> {
        Self::obj().call("ownKeys", &[target.clone()]).as_()
    }

    pub fn prevent_extensions(target: &Any) -> bool {
        Self::obj()
            .call("preventExtensions", &[target.clone()])
            .as_()
    }

    pub fn set_prototype_of(target: &Any, proto: &Any) -> bool {
        Self::obj()
            .call("setPrototypeOf", &[target.clone(), proto.clone()])
            .as_()
    }
}
