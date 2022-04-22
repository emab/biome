use crate::utils::sort_modifiers_by_precedence;
use crate::{join_elements, space_token, Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsPropertyParameterModifierList;

impl Format for TsPropertyParameterModifierList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(sort_modifiers_by_precedence(self))?,
        ))
    }
}
