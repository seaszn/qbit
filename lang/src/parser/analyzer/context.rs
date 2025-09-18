use std::ops::Range;

#[derive(Debug, Clone)]
pub struct ParseContext {
    pub line_number: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub line_content: String,
    pub span_in_line: Range<usize>,
}

impl ParseContext {
    pub fn from_span(source: &str, span: &Range<usize>) -> Self {
        let lines: Vec<&str> = source.lines().collect();
        let mut current_pos = 0;

        for (line_num, line) in lines.iter().enumerate() {
            let line_start = current_pos;
            let line_end = current_pos + line.len();

            if span.start >= line_start && span.start <= line_end {
                let col_start = span.start - line_start;
                let col_end = (span.end - line_start).min(line.len());

                return Self {
                    line_number: line_num + 1,
                    column_start: col_start + 1,
                    column_end: col_end + 1,
                    line_content: line.to_string(),
                    span_in_line: col_start..col_end,
                };
            }

            current_pos = line_end + 1;
        }

        Self {
            line_number: lines.len(),
            column_start: 1,
            column_end: 1,
            line_content: lines.last().unwrap_or(&"").to_string(),
            span_in_line: 0..0,
        }
    }
}

impl std::fmt::Display for ParseContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.span_in_line.is_empty() {
            true => write!(
                f,
                "{}:{}: {}",
                self.line_number, self.column_start, self.line_content
            ),
            false => {
                let caret_line = format!(
                    "{}{}",
                    " ".repeat(self.span_in_line.start),
                    "^".repeat((self.span_in_line.end - self.span_in_line.start).max(1))
                );

                write!(
                    f,
                    "{}:{}-{}: {}\n{}",
                    self.line_number,
                    self.column_start,
                    self.column_end,
                    self.line_content,
                    caret_line
                )
            }
        }
    }
}
