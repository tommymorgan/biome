use crate::prelude::*;

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::JsStringLiteralExpression;
use biome_js_syntax::JsStringLiteralExpressionFields;
use biome_js_syntax::{JsExpressionStatement, JsSyntaxKind};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsStringLiteralExpression;

impl FormatNodeRule<JsStringLiteralExpression> for FormatJsStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsStringLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        let formatted =
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression);

        formatted.fmt(f)
    }

    fn needs_parentheses(&self, item: &JsStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsStringLiteralExpression {
    fn needs_parentheses(&self) -> bool {
        if let Some(expression_statement) = self.parent::<JsExpressionStatement>() {
            expression_statement
                .syntax()
                .parent()
                .is_some_and(|grand_parent| {
                    matches!(
                        grand_parent.kind(),
                        JsSyntaxKind::JS_STATEMENT_LIST | JsSyntaxKind::JS_MODULE_ITEM_LIST
                    )
                })
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::{JsFileSource, JsStringLiteralExpression, ModuleKind};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("{ 'test'; }", JsStringLiteralExpression);
        assert_needs_parentheses!(
            r#"
            {
                console.log(5);
                'test';
            }
            "#,
            JsStringLiteralExpression
        );
        assert_needs_parentheses!(
            r#"
            function Test () {
                ('test');
            }
            "#,
            JsStringLiteralExpression
        );
        assert_needs_parentheses!(
            r#"
            class A {
                static {
                    ('test');
                }
            }
            "#,
            JsStringLiteralExpression
        );
        assert_needs_parentheses!(
            "('test');",
            JsStringLiteralExpression,
            JsFileSource::ts().with_module_kind(ModuleKind::Module)
        );

        assert_not_needs_parentheses!("console.log('a')", JsStringLiteralExpression);
    }
}
