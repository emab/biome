use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsRegexLiteralExpression;
use rome_js_syntax::JsRegexLiteralExpressionFields;

impl FormatNode for JsRegexLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsRegexLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
