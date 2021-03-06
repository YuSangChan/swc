use ast::*;
use swc_common::{Fold, FoldWith};

/// Compile ES2015 shorthand properties to ES5
///
///# Example
///
///## In
///
/// ```js
/// var o = { a, b, c };
/// ```
///
///## Out
///
/// ```js
/// var o = { a: a, b: b, c: c };
/// ```
///
///## In
///
/// ```js
/// var cat = {
///   getName() {
///     return name;
///   }
/// };
/// ```
///
///## Out
///```js
/// var cat = {
///   getName: function () {
///     return name;
///   }
/// };
/// ```
#[derive(Default)]
pub struct Shorthand;

impl Fold<Prop> for Shorthand {
    fn fold(&mut self, prop: Prop) -> Prop {
        let prop = prop.fold_children(self);

        match prop {
            Prop::Shorthand(Ident { sym, span, .. }) => Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(quote_ident!(span, sym.clone())),
                value: box quote_ident!(span, sym).into(),
            }),
            Prop::Method(MethodProp { key, function }) => Prop::KeyValue(KeyValueProp {
                key,
                value: box Expr::Fn(FnExpr {
                    ident: None,
                    function,
                }),
            }),
            _ => prop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test!(
        ::swc_ecma_parser::Syntax::default(),
        Shorthand,
        babel_method_plain,
        "var obj = {
  method() {
    return 5 + 5;
  }
};",
        "var obj = {
  method: function () {
    return 5 + 5;
  }
};"
    );

    test!(
        ::swc_ecma_parser::Syntax::default(),
        Shorthand,
        babel_comments,
        "var A = 'a';
var o = {
  A // comment
};",
        "var A = 'a';
var o = {
  A: A // comment

};"
    );

    test!(
        ::swc_ecma_parser::Syntax::default(),
        Shorthand,
        babel_mixed,
        "var coords = { x, y, foo: 'bar' };",
        "var coords = {
  x: x,
  y: y,
  foo: 'bar'
};"
    );

    test!(
        ::swc_ecma_parser::Syntax::default(),
        Shorthand,
        babel_multiple,
        "var coords = { x, y };",
        "var coords = {
  x: x,
  y: y
};"
    );

    test!(
        ::swc_ecma_parser::Syntax::default(),
        Shorthand,
        babel_single,
        "var coords = { x };",
        "var coords = {
  x: x
};"
    );

}
