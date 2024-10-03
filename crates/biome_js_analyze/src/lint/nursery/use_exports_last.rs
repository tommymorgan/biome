use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsModuleItem, JsModuleItemList, JsSyntaxNode};
use biome_rowan::AstNode;
use biome_rowan::AstNodeList;

declare_lint_rule! {
    /// Require that all exports are declared after all non-export statements.
    ///
    /// Enforces that export statements are placed at the end of the module, after all other statements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export const a = 1;
    /// const b = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 1;
    /// export const b = 2;
    /// ```
    ///
    /// ```js
    /// const a = 1;
    /// export { a };
    /// ```
    ///
    pub UseExportsLast {
        version: "next",
        name: "useExportsLast",
        language: "js",
        recommended: false,
    }
}

impl Rule for UseExportsLast {
    type Query = Ast<JsModuleItemList>;
    type State = JsSyntaxNode;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let items = ctx.query();
        let mut last_export = None;

        for item in items.iter() {
            if matches!(item, AnyJsModuleItem::JsExport(_)) {
                last_export = Some(item.syntax().clone());
            } else if last_export.is_some() {
                return last_export;
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.text_range(),
                markup! {
                    "Export statements should appear at the end of the file."
                },
            )
            .note(markup! {
                "Move this export to the end of the file."
            }),
        )
    }
}
