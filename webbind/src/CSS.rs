use super::*;

/// The escape function from the CSS namespace.
pub fn escape(ident: &JsString) -> JsString {
    Any::global("CSS")
        .call("escape", &[ident.into()])
        .as_::<JsString>()
}

/// The supports function from the CSS namespace.
pub fn supports(condition_text: &JsString) -> bool {
    Any::global("CSS")
        .call("supports", &[condition_text.into()])
        .as_::<bool>()
}

/// The parseStylesheet function from the CSS namespace.
pub fn parse_stylesheet0(css: &Any) -> Promise<TypedArray<CSSParserRule>> {
    Any::global("CSS")
        .call("parseStylesheet", &[css.into()])
        .as_::<Promise<TypedArray<CSSParserRule>>>()
}

/// The parseStylesheet function from the CSS namespace.
pub fn parse_stylesheet1(
    css: &Any,
    options: &CSSParserOptions,
) -> Promise<TypedArray<CSSParserRule>> {
    Any::global("CSS")
        .call("parseStylesheet", &[css.into(), options.into()])
        .as_::<Promise<TypedArray<CSSParserRule>>>()
}

/// The parseRuleList function from the CSS namespace.
pub fn parse_rule_list0(css: &Any) -> Promise<TypedArray<CSSParserRule>> {
    Any::global("CSS")
        .call("parseRuleList", &[css.into()])
        .as_::<Promise<TypedArray<CSSParserRule>>>()
}

/// The parseRuleList function from the CSS namespace.
pub fn parse_rule_list1(
    css: &Any,
    options: &CSSParserOptions,
) -> Promise<TypedArray<CSSParserRule>> {
    Any::global("CSS")
        .call("parseRuleList", &[css.into(), options.into()])
        .as_::<Promise<TypedArray<CSSParserRule>>>()
}

/// The parseRule function from the CSS namespace.
pub fn parse_rule0(css: &Any) -> Promise<CSSParserRule> {
    Any::global("CSS")
        .call("parseRule", &[css.into()])
        .as_::<Promise<CSSParserRule>>()
}

/// The parseRule function from the CSS namespace.
pub fn parse_rule1(css: &Any, options: &CSSParserOptions) -> Promise<CSSParserRule> {
    Any::global("CSS")
        .call("parseRule", &[css.into(), options.into()])
        .as_::<Promise<CSSParserRule>>()
}

/// The parseDeclarationList function from the CSS namespace.
pub fn parse_declaration_list0(css: &Any) -> Promise<TypedArray<CSSParserRule>> {
    Any::global("CSS")
        .call("parseDeclarationList", &[css.into()])
        .as_::<Promise<TypedArray<CSSParserRule>>>()
}

/// The parseDeclarationList function from the CSS namespace.
pub fn parse_declaration_list1(
    css: &Any,
    options: &CSSParserOptions,
) -> Promise<TypedArray<CSSParserRule>> {
    Any::global("CSS")
        .call("parseDeclarationList", &[css.into(), options.into()])
        .as_::<Promise<TypedArray<CSSParserRule>>>()
}

/// The parseDeclaration function from the CSS namespace.
pub fn parse_declaration0(css: &JsString) -> CSSParserDeclaration {
    Any::global("CSS")
        .call("parseDeclaration", &[css.into()])
        .as_::<CSSParserDeclaration>()
}

/// The parseDeclaration function from the CSS namespace.
pub fn parse_declaration1(css: &JsString, options: &CSSParserOptions) -> CSSParserDeclaration {
    Any::global("CSS")
        .call("parseDeclaration", &[css.into(), options.into()])
        .as_::<CSSParserDeclaration>()
}

/// The parseValue function from the CSS namespace.
pub fn parse_value(css: &JsString) -> Any {
    Any::global("CSS")
        .call("parseValue", &[css.into()])
        .as_::<Any>()
}

/// The parseValueList function from the CSS namespace.
pub fn parse_value_list(css: &JsString) -> TypedArray<Any> {
    Any::global("CSS")
        .call("parseValueList", &[css.into()])
        .as_::<TypedArray<Any>>()
}

/// The parseCommaValueList function from the CSS namespace.
pub fn parse_comma_value_list(css: &JsString) -> TypedArray<TypedArray<Any>> {
    Any::global("CSS")
        .call("parseCommaValueList", &[css.into()])
        .as_::<TypedArray<TypedArray<Any>>>()
}

/// The registerProperty function from the CSS namespace.
pub fn register_property(definition: &PropertyDefinition) -> Undefined {
    Any::global("CSS")
        .call("registerProperty", &[definition.into()])
        .as_::<Undefined>()
}

/// The number function from the CSS namespace.
pub fn number(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("number", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The percent function from the CSS namespace.
pub fn percent(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("percent", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The cap function from the CSS namespace.
pub fn cap(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("cap", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The ch function from the CSS namespace.
pub fn ch(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("ch", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The em function from the CSS namespace.
pub fn em(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("em", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The ex function from the CSS namespace.
pub fn ex(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("ex", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The ic function from the CSS namespace.
pub fn ic(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("ic", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The lh function from the CSS namespace.
pub fn lh(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("lh", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The rcap function from the CSS namespace.
pub fn rcap(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("rcap", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The rch function from the CSS namespace.
pub fn rch(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("rch", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The rem function from the CSS namespace.
pub fn rem(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("rem", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The rex function from the CSS namespace.
pub fn rex(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("rex", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The ric function from the CSS namespace.
pub fn ric(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("ric", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The rlh function from the CSS namespace.
pub fn rlh(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("rlh", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The vw function from the CSS namespace.
pub fn vw(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("vw", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The vh function from the CSS namespace.
pub fn vh(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("vh", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The vi function from the CSS namespace.
pub fn vi(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("vi", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The vb function from the CSS namespace.
pub fn vb(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("vb", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The vmin function from the CSS namespace.
pub fn vmin(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("vmin", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The vmax function from the CSS namespace.
pub fn vmax(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("vmax", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The svw function from the CSS namespace.
pub fn svw(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("svw", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The svh function from the CSS namespace.
pub fn svh(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("svh", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The svi function from the CSS namespace.
pub fn svi(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("svi", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The svb function from the CSS namespace.
pub fn svb(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("svb", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The svmin function from the CSS namespace.
pub fn svmin(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("svmin", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The svmax function from the CSS namespace.
pub fn svmax(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("svmax", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The lvw function from the CSS namespace.
pub fn lvw(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("lvw", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The lvh function from the CSS namespace.
pub fn lvh(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("lvh", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The lvi function from the CSS namespace.
pub fn lvi(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("lvi", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The lvb function from the CSS namespace.
pub fn lvb(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("lvb", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The lvmin function from the CSS namespace.
pub fn lvmin(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("lvmin", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The lvmax function from the CSS namespace.
pub fn lvmax(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("lvmax", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dvw function from the CSS namespace.
pub fn dvw(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dvw", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dvh function from the CSS namespace.
pub fn dvh(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dvh", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dvi function from the CSS namespace.
pub fn dvi(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dvi", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dvb function from the CSS namespace.
pub fn dvb(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dvb", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dvmin function from the CSS namespace.
pub fn dvmin(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dvmin", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dvmax function from the CSS namespace.
pub fn dvmax(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dvmax", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The cqw function from the CSS namespace.
pub fn cqw(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("cqw", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The cqh function from the CSS namespace.
pub fn cqh(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("cqh", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The cqi function from the CSS namespace.
pub fn cqi(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("cqi", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The cqb function from the CSS namespace.
pub fn cqb(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("cqb", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The cqmin function from the CSS namespace.
pub fn cqmin(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("cqmin", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The cqmax function from the CSS namespace.
pub fn cqmax(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("cqmax", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The cm function from the CSS namespace.
pub fn cm(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("cm", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The mm function from the CSS namespace.
pub fn mm(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("mm", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The Q function from the CSS namespace.
pub fn q(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("Q", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The in function from the CSS namespace.
pub fn in_(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("in", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The pt function from the CSS namespace.
pub fn pt(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("pt", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The pc function from the CSS namespace.
pub fn pc(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("pc", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The px function from the CSS namespace.
pub fn px(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("px", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The deg function from the CSS namespace.
pub fn deg(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("deg", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The grad function from the CSS namespace.
pub fn grad(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("grad", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The rad function from the CSS namespace.
pub fn rad(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("rad", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The turn function from the CSS namespace.
pub fn turn(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("turn", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The s function from the CSS namespace.
pub fn s(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("s", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The ms function from the CSS namespace.
pub fn ms(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("ms", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The Hz function from the CSS namespace.
pub fn hz(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("Hz", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The kHz function from the CSS namespace.
pub fn k_hz(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("kHz", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dpi function from the CSS namespace.
pub fn dpi(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dpi", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dpcm function from the CSS namespace.
pub fn dpcm(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dpcm", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The dppx function from the CSS namespace.
pub fn dppx(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("dppx", &[value.into()])
        .as_::<CSSUnitValue>()
}

/// The fr function from the CSS namespace.
pub fn fr(value: f64) -> CSSUnitValue {
    Any::global("CSS")
        .call("fr", &[value.into()])
        .as_::<CSSUnitValue>()
}
