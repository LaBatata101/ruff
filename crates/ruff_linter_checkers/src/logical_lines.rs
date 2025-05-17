//FIX: duplicated code with ruff_linter::checkers::logical_lines

use ruff_codes::AsRule;
use ruff_diagnostics::Diagnostic;
use ruff_linter_commons::line_width::IndentWidth;
use ruff_linter_settings::LinterSettings;

/// Return the amount of indentation, expanding tabs to the next multiple of the settings' tab size.
pub fn expand_indent(line: &str, indent_width: IndentWidth) -> usize {
    let line = line.trim_end_matches(['\n', '\r']);

    let mut indent = 0;
    let tab_size = indent_width.as_usize();
    for c in line.bytes() {
        match c {
            b'\t' => indent = (indent / tab_size) * tab_size + tab_size,
            b' ' => indent += 1,
            _ => break,
        }
    }

    indent
}

#[derive(Debug, Clone)]
pub struct LogicalLinesContext<'a> {
    // FIX: temporary pub
    pub settings: &'a LinterSettings,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a> LogicalLinesContext<'a> {
    pub fn new(settings: &'a LinterSettings) -> Self {
        Self {
            settings,
            diagnostics: Vec::new(),
        }
    }

    pub fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        if self.settings.rules.enabled(diagnostic.rule()) {
            self.diagnostics.push(diagnostic);
        }
    }
}
