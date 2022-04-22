use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsxOpeningElement;
use rome_rowan::AstNode;

impl FormatNode for JsxOpeningElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
