#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathBlock {
    pub content: String,
    pub line_number: usize,
    pub is_block: bool,
}

pub struct DocLine {
    pub line_number: usize,
    pub text: String,
}

/// Helper to determine if a character in a char slice is escaped by backslashes
pub fn is_escaped_char(chars: &[char], pos: usize) -> bool {
    let mut backslash_count = 0;
    let mut idx = pos;
    while idx > 0 {
        idx -= 1;
        if chars[idx] == '\\' {
            backslash_count += 1;
        } else {
            break;
        }
    }
    backslash_count % 2 == 1
}

/// Extracts LaTeX math blocks (demarcated by block or inline delimiters) from markdown documentation.
pub fn extract_markdown_math(content: &str) -> Vec<MathBlock> {
    let mut blocks = Vec::new();
    let chars = content.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut line = 1;

    let mut in_code_block = false;
    let mut in_inline_code = false;
    let mut inline_code_backticks_count = 0;

    let mut in_block_math = false;
    let mut block_math_start_line = 0;
    let mut block_math_content = String::new();

    let mut in_inline_math = false;
    let mut inline_math_start_line = 0;
    let mut inline_math_content = String::new();

    while i < chars.len() {
        let c = chars[i];

        // Handle newlines
        if c == '\n' {
            line += 1;
        }

        // 1. Check for fenced code blocks (triple backticks)
        if !in_inline_code && !in_block_math && !in_inline_math {
            if i + 2 < chars.len() && chars[i] == '`' && chars[i + 1] == '`' && chars[i + 2] == '`'
            {
                in_code_block = !in_code_block;
                i += 3;
                continue;
            }
        }

        if in_code_block {
            i += 1;
            continue;
        }

        // 2. Check for inline code blocks (backticks)
        if !in_block_math && !in_inline_math {
            if c == '`' {
                let mut count = 0;
                let mut temp_i = i;
                while temp_i < chars.len() && chars[temp_i] == '`' {
                    count += 1;
                    temp_i += 1;
                }
                if in_inline_code {
                    if count == inline_code_backticks_count {
                        in_inline_code = false;
                        i += count;
                        continue;
                    }
                } else {
                    in_inline_code = true;
                    inline_code_backticks_count = count;
                    i += count;
                    continue;
                }
            }
        }

        if in_inline_code {
            i += 1;
            continue;
        }

        // 3. Check for math blocks
        if in_block_math {
            if i + 1 < chars.len()
                && chars[i] == '$'
                && chars[i + 1] == '$'
                && !is_escaped_char(&chars, i)
            {
                blocks.push(MathBlock {
                    content: block_math_content.clone(),
                    line_number: block_math_start_line,
                    is_block: true,
                });
                block_math_content.clear();
                in_block_math = false;
                i += 2;
                continue;
            } else {
                block_math_content.push(c);
                i += 1;
                continue;
            }
        }

        if in_inline_math {
            if c == '$' && !is_escaped_char(&chars, i) {
                blocks.push(MathBlock {
                    content: inline_math_content.clone(),
                    line_number: inline_math_start_line,
                    is_block: false,
                });
                inline_math_content.clear();
                in_inline_math = false;
                i += 1;
                continue;
            } else {
                inline_math_content.push(c);
                i += 1;
                continue;
            }
        }

        // Check for start of math block (not escaped)
        if i + 1 < chars.len()
            && chars[i] == '$'
            && chars[i + 1] == '$'
            && !is_escaped_char(&chars, i)
        {
            in_block_math = true;
            block_math_start_line = line;
            i += 2;
            continue;
        }

        if c == '$' && !is_escaped_char(&chars, i) {
            in_inline_math = true;
            inline_math_start_line = line;
            i += 1;
            continue;
        }

        i += 1;
    }

    if in_block_math {
        blocks.push(MathBlock {
            content: "UNMATCHED_BLOCK_MATH_DELIMITER_ERROR".to_string(),
            line_number: block_math_start_line,
            is_block: true,
        });
    } else if in_inline_math {
        blocks.push(MathBlock {
            content: "UNMATCHED_INLINE_MATH_DELIMITER_ERROR".to_string(),
            line_number: inline_math_start_line,
            is_block: false,
        });
    }

    blocks
}

/// Helper to process a grouped chunk of Rust docstrings
fn process_doc_group(group: &[DocLine], blocks: &mut Vec<MathBlock>) {
    if group.is_empty() {
        return;
    }
    let joined = group
        .iter()
        .map(|dl| dl.text.as_str())
        .collect::<Vec<&str>>()
        .join("\n");
    let relative_blocks = extract_markdown_math(&joined);
    for mut rb in relative_blocks {
        if rb.line_number > 0 && rb.line_number <= group.len() {
            rb.line_number = group[rb.line_number - 1].line_number;
        } else {
            rb.line_number = group[0].line_number;
        }
        blocks.push(rb);
    }
}

/// Extracts LaTeX math blocks from inline Rust docstrings.
pub fn extract_rust_math(content: &str) -> Vec<MathBlock> {
    let mut blocks = Vec::new();
    let mut current_doc_group: Vec<DocLine> = Vec::new();

    for (zero_idx, line) in content.lines().enumerate() {
        let line_number = zero_idx + 1;
        let trimmed = line.trim_start();

        let is_doc = if trimmed.starts_with("///") {
            !trimmed.starts_with("////")
        } else {
            trimmed.starts_with("//!")
        };

        if is_doc {
            let prefix = if trimmed.starts_with("///") {
                "///"
            } else {
                "//!"
            };
            let rest = &trimmed[prefix.len()..];
            let stripped = if rest.starts_with(' ') {
                &rest[1..]
            } else {
                rest
            };
            current_doc_group.push(DocLine {
                line_number,
                text: stripped.to_string(),
            });
        } else {
            if !current_doc_group.is_empty() {
                process_doc_group(&current_doc_group, &mut blocks);
                current_doc_group.clear();
            }
        }
    }

    if !current_doc_group.is_empty() {
        process_doc_group(&current_doc_group, &mut blocks);
    }

    blocks
}
