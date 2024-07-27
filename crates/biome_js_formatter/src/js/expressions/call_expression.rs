use crate::prelude::*;
use biome_formatter::write;

use crate::parentheses::{resolve_left_most_expression, NeedsParentheses};
use crate::utils::member_chain::MemberChain;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsCallExpressionFields, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsCallExpression;

impl FormatNodeRule<JsCallExpression> for FormatJsCallExpression {
    fn fmt_fields(&self, node: &JsCallExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCallExpressionFields {
            callee,
            optional_chain_token,
            type_arguments,
            arguments,
        } = node.as_fields();

        let callee = callee?;

        if matches!(
            callee,
            AnyJsExpression::JsStaticMemberExpression(_)
                | AnyJsExpression::JsComputedMemberExpression(_)
        ) && !callee.needs_parentheses()
        {
            let member_chain = MemberChain::from_call_expression(
                node.clone(),
                f.comments(),
                f.options().tab_width(),
            )?;

            member_chain.fmt(f)
        } else {
            let format_inner = format_with(|f| {
                write!(
                    f,
                    [
                        callee.format(),
                        optional_chain_token.format(),
                        type_arguments.format(),
                        arguments.format()
                    ]
                )
            });

            if matches!(callee, AnyJsExpression::JsCallExpression(_)) {
                write!(f, [group(&format_inner)])
            } else {
                write!(f, [format_inner])
            }
        }
    }

    fn needs_parentheses(&self, item: &JsCallExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsCallExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        parent.kind() == JsSyntaxKind::JS_NEW_EXPRESSION
            || (parent.kind() == JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
                && self.callee().map_or(true, |callee| {
                    let callee_range = callee.range();
                    let leftmost = resolve_left_most_expression(callee);
                    // require parens for iife and
                    // when the leftmost expression is not a class expression or a function expression
                    callee_range != leftmost.range()
                        && matches!(
                            leftmost.syntax().kind(),
                            JsSyntaxKind::JS_CLASS_EXPRESSION
                                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                        )
                }))
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsCallExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("new (call())()", JsCallExpression);

        assert_not_needs_parentheses!("a?.()!.c", JsCallExpression);
        assert_not_needs_parentheses!("(a?.())!.c", JsCallExpression);

        assert_not_needs_parentheses!("(call())()", JsCallExpression[1]);
        assert_not_needs_parentheses!("getLogger().error(err);", JsCallExpression[0]);
        assert_not_needs_parentheses!("getLogger().error(err);", JsCallExpression[1]);
    }
}
