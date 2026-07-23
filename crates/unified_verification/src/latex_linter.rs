use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathBlock {
    pub content: String,
    pub line_number: usize,
    pub is_block: bool,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub details: String,
}

#[derive(Debug, Clone)]
pub struct LinterError {
    pub file_path: String,
    pub line_number: usize,
    pub error: ValidationError,
    pub context_line: String,
}

struct DocLine {
    line_number: usize,
    text: String,
}

/// Helper to determine if a character in a char slice is escaped by backslashes
fn is_escaped_char(chars: &[char], pos: usize) -> bool {
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

/// Helper to determine if a character at byte index `pos` in string `s` is escaped
fn is_escaped(s: &str, pos: usize) -> bool {
    let bytes = s.as_bytes();
    let mut backslash_count = 0;
    let mut idx = pos;
    while idx > 0 {
        idx -= 1;
        if bytes[idx] == b'\\' {
            backslash_count += 1;
        } else {
            break;
        }
    }
    backslash_count % 2 == 1
}

/// Strip LaTeX comments from raw LaTeX string. Preserves line count by outputting newlines.
pub fn strip_latex_comments(latex: &str) -> String {
    let mut result = String::new();
    for line in latex.lines() {
        let mut clean_line = String::new();
        let chars = line.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '%' && !is_escaped_char(&chars, i) {
                break;
            }
            clean_line.push(chars[i]);
            i += 1;
        }
        result.push_str(&clean_line);
        result.push('\n');
    }
    result
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
            if i + 2 < chars.len() && chars[i] == '`' && chars[i+1] == '`' && chars[i+2] == '`' {
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
            if i + 1 < chars.len() && chars[i] == '$' && chars[i+1] == '$' && !is_escaped_char(&chars, i) {
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
        if i + 1 < chars.len() && chars[i] == '$' && chars[i+1] == '$' && !is_escaped_char(&chars, i) {
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
    let joined = group.iter().map(|dl| dl.text.as_str()).collect::<Vec<&str>>().join("\n");
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
            let prefix = if trimmed.starts_with("///") { "///" } else { "//!" };
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

/// Validates LaTeX math block syntax
pub fn validate_latex(raw_latex: &str) -> Result<(), ValidationError> {
    if raw_latex == "UNMATCHED_BLOCK_MATH_DELIMITER_ERROR" {
        return Err(ValidationError {
            message: "Unmatched math block delimiter".to_string(),
            details: "A block math delimiter '$$' was opened but never closed.".to_string(),
        });
    }
    if raw_latex == "UNMATCHED_INLINE_MATH_DELIMITER_ERROR" {
        return Err(ValidationError {
            message: "Unmatched inline math delimiter".to_string(),
            details: "An inline math delimiter '$' was opened but never closed.".to_string(),
        });
    }

    let latex = strip_latex_comments(raw_latex);

    // 1. Check for balanced braces { } (ignoring escaped ones)
    let mut brace_level = 0;
    for (idx, c) in latex.char_indices() {
        if c == '{' && !is_escaped(&latex, idx) {
            brace_level += 1;
        } else if c == '}' && !is_escaped(&latex, idx) {
            brace_level -= 1;
            if brace_level < 0 {
                return Err(ValidationError {
                    message: "Unbalanced curly braces".to_string(),
                    details: "Found closing brace '}' without a matching opening brace '{'.".to_string(),
                });
            }
        }
    }
    if brace_level > 0 {
        return Err(ValidationError {
            message: "Unbalanced curly braces".to_string(),
            details: format!("Found {} unmatched opening brace(s) '{{'.", brace_level),
        });
    }

    // 2. Check for begin/end environments matching
    let env_regex = regex::Regex::new(r"\\(begin|end)\{([^}]+)\}").unwrap();
    let mut env_stack = Vec::new();
    for mat in env_regex.captures_iter(&latex) {
        let action = mat.get(1).unwrap().as_str();
        let env_name = mat.get(2).unwrap().as_str();
        if action == "begin" {
            env_stack.push(env_name.to_string());
        } else {
            match env_stack.pop() {
                Some(pushed_name) => {
                    if pushed_name != env_name {
                        return Err(ValidationError {
                            message: "Mismatched environments".to_string(),
                            details: format!(
                                "LaTeX environment mismatch: started with '\\begin{{{}}}' but closed with '\\end{{{}}}'.",
                                pushed_name, env_name
                            ),
                        });
                    }
                }
                None => {
                    return Err(ValidationError {
                        message: "Unmatched environment closure".to_string(),
                        details: format!("Found '\\end{{{}}}' without a corresponding '\\begin'.", env_name),
                    });
                }
            }
        }
    }
    if let Some(leftover_name) = env_stack.pop() {
        return Err(ValidationError {
            message: "Unclosed environment".to_string(),
            details: format!(
                "LaTeX environment '\\begin{{{}}}' was never closed with '\\end{{{}}}'.",
                leftover_name, leftover_name
            ),
        });
    }

    // 3. Check for matched \left and \right
    let left_right_regex = regex::Regex::new(r"\\(left|right)\b").unwrap();
    let mut left_count = 0;
    for mat in left_right_regex.captures_iter(&latex) {
        let cmd = mat.get(1).unwrap().as_str();
        if cmd == "left" {
            left_count += 1;
        } else {
            left_count -= 1;
            if left_count < 0 {
                return Err(ValidationError {
                    message: "Unmatched \\right".to_string(),
                    details: "Found '\\right' without a corresponding '\\left'.".to_string(),
                });
            }
        }
    }
    if left_count > 0 {
        return Err(ValidationError {
            message: "Unmatched \\left".to_string(),
            details: format!("Found {} unmatched '\\left' command(s).", left_count),
        });
    }

    // 4. Check for trailing backslash escape
    let trimmed = latex.trim_end();
    if trimmed.ends_with('\\') && !trimmed.ends_with("\\\\") {
        return Err(ValidationError {
            message: "Trailing backslash escape".to_string(),
            details: "Math block ends with a single backslash, which is an invalid escape/command.".to_string(),
        });
    }

    Ok(())
}

fn get_context_line(path: &Path, line_number: usize) -> String {
    if let Ok(content) = fs::read_to_string(path) {
        if line_number > 0 {
            if let Some(line) = content.lines().nth(line_number - 1) {
                return line.to_string();
            }
        }
    }
    String::new()
}

fn process_markdown_file(path: &Path) -> Result<(), Vec<LinterError>> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };

    let blocks = extract_markdown_math(&content);
    let mut file_errors = Vec::new();

    for block in blocks {
        if let Err(err) = validate_latex(&block.content) {
            let context = get_context_line(path, block.line_number);
            file_errors.push(LinterError {
                file_path: path.to_string_lossy().to_string(),
                line_number: block.line_number,
                error: err,
                context_line: context,
            });
        }
    }

    if file_errors.is_empty() {
        Ok(())
    } else {
        Err(file_errors)
    }
}

fn process_rust_file(path: &Path) -> Result<(), Vec<LinterError>> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };

    let blocks = extract_rust_math(&content);
    let mut file_errors = Vec::new();

    for block in blocks {
        if let Err(err) = validate_latex(&block.content) {
            let context = get_context_line(path, block.line_number);
            file_errors.push(LinterError {
                file_path: path.to_string_lossy().to_string(),
                line_number: block.line_number,
                error: err,
                context_line: context,
            });
        }
    }

    if file_errors.is_empty() {
        Ok(())
    } else {
        Err(file_errors)
    }
}

/// Scans the workspace and validates all math blocks
pub fn lint_latex() -> bool {
    println!("=== Running Unified LaTeX Math Linter ===");
    let mut errors = Vec::new();

    let walker = WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            if name == "." {
                return true;
            }
            name != "target" && name != ".git" && name != ".cargo" && !name.starts_with('.')
        });

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path.extension().and_then(|s| s.to_str());
        if ext == Some("md") {
            if let Err(errs) = process_markdown_file(path) {
                errors.extend(errs);
            }
        } else if ext == Some("rs") {
            if let Err(errs) = process_rust_file(path) {
                errors.extend(errs);
            }
        }
    }

    if errors.is_empty() {
        println!("LaTeX Linter: SUCCESS. All math blocks are syntactically valid.");
        true
    } else {
        eprintln!("\nLaTeX Linter: Found {} syntax error(s) in mathematical formulas:", errors.len());
        for err in &errors {
            eprintln!(
                "Error in file: {} at line {}\n  Context: {}\n  Error Type: {}\n  Details: {}\n",
                err.file_path,
                err.line_number,
                err.context_line.trim(),
                err.error.message,
                err.error.details
            );
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_latex_valid() {
        assert!(validate_latex("a + b = c").is_ok());
        assert!(validate_latex("\\frac{a}{b}").is_ok());
        assert!(validate_latex("\\begin{matrix} a & b \\\\ c & d \\end{matrix}").is_ok());
        assert!(validate_latex("\\left( x + y \\right)").is_ok());
        assert!(validate_latex("a \\% b").is_ok());
        assert!(validate_latex("a % comment here\n+ b").is_ok());
    }

    #[test]
    fn test_validate_latex_invalid_braces() {
        let err = validate_latex("\\frac{a").unwrap_err();
        assert_eq!(err.message, "Unbalanced curly braces");

        let err2 = validate_latex("a}").unwrap_err();
        assert_eq!(err2.message, "Unbalanced curly braces");
    }

    #[test]
    fn test_validate_latex_invalid_environments() {
        let err = validate_latex("\\begin{matrix} a \\end{pmatrix}").unwrap_err();
        assert_eq!(err.message, "Mismatched environments");

        let err2 = validate_latex("\\end{matrix}").unwrap_err();
        assert_eq!(err2.message, "Unmatched environment closure");

        let err3 = validate_latex("\\begin{matrix} a").unwrap_err();
        assert_eq!(err3.message, "Unclosed environment");
    }

    #[test]
    fn test_validate_latex_invalid_left_right() {
        let err = validate_latex("\\left( x").unwrap_err();
        assert_eq!(err.message, "Unmatched \\left");

        let err2 = validate_latex("x \\right)").unwrap_err();
        assert_eq!(err2.message, "Unmatched \\right");
    }

    #[test]
    fn test_validate_latex_trailing_escape() {
        let err = validate_latex("a \\").unwrap_err();
        assert_eq!(err.message, "Trailing backslash escape");
    }

    #[test]
    fn test_extract_markdown_math() {
        let md = "Some text with $a + b$ and a block:\n\n$$\nx^2 + y^2 = z^2\n$$\nCode block:\n```\n$this is code$\n```\nAnd `inline code $math$`";
        let blocks = extract_markdown_math(md);
        assert_eq!(blocks.len(), 2);
        
        assert_eq!(blocks[0].content, "a + b");
        assert_eq!(blocks[0].is_block, false);
        assert_eq!(blocks[0].line_number, 1);

        assert_eq!(blocks[1].content, "\nx^2 + y^2 = z^2\n");
        assert_eq!(blocks[1].is_block, true);
        assert_eq!(blocks[1].line_number, 3);
    }

    #[test]
    fn test_extract_rust_math() {
        let code = "
/// This is a doc comment with $V_m$ formula.
///
/// $$
/// C_m = I
/// $$
fn test_func() {}
";
        let blocks = extract_rust_math(code);
        assert_eq!(blocks.len(), 2);

        assert_eq!(blocks[0].content, "V_m");
        assert_eq!(blocks[0].is_block, false);
        assert_eq!(blocks[0].line_number, 2);

        assert_eq!(blocks[1].content, "\nC_m = I\n");
        assert_eq!(blocks[1].is_block, true);
        assert_eq!(blocks[1].line_number, 4);
    }
}
