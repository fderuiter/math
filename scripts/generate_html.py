import os
import re
import latex2mathml.converter

def tex_to_html(tex_content):
    def inline_repl(match):
        math_expr = match.group(1)
        try:
            return latex2mathml.converter.convert(math_expr)
        except Exception as e:
            print(f"Error converting {math_expr}: {e}")
            return match.group(0)
    
    html = re.sub(r'\$([^$]+)\$', inline_repl, tex_content)
    
    def display_repl(match):
        math_expr = match.group(1)
        try:
            return latex2mathml.converter.convert(math_expr)
        except Exception as e:
            return match.group(0)
            
    html = re.sub(r'\\begin\{equation\}(.*?)\\end\{equation\}', display_repl, html, flags=re.DOTALL)
    html = re.sub(r'\\\[(.*?)\\\]', display_repl, html, flags=re.DOTALL)
    
    # Basic structural replacements
    html = re.sub(r'\\section\{(.*?)\}', r'<h2>\1</h2>', html)
    html = re.sub(r'\\subsection\{(.*?)\}', r'<h3>\1</h3>', html)
    html = re.sub(r'\\title\{(.*?)\}', r'<h1>\1</h1>', html)
    html = re.sub(r'\\author\{(.*?)\}', r'<p><strong>Author:</strong> \1</p>', html)
    html = re.sub(r'\\begin\{abstract\}', r'<div class="abstract"><strong>Abstract:</strong><p>', html)
    html = re.sub(r'\\end\{abstract\}', r'</p></div>', html)
    
    # Strip some common preamble commands
    html = re.sub(r'\\documentclass\[.*?\]\{.*?\}', '', html)
    html = re.sub(r'\\documentclass\{.*?\}', '', html)
    html = re.sub(r'\\usepackage\[.*?\]\{.*?\}', '', html)
    html = re.sub(r'\\usepackage\{.*?\}', '', html)
    html = re.sub(r'\\maketitle', '', html)
    html = re.sub(r'\\date\{.*?\}', '', html)
    
    html = re.sub(r'\\begin\{document\}', r'<body>', html)
    html = re.sub(r'\\end\{document\}', r'</body>', html)
    
    html_template = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Paper</title>
<style>
  body {{ max-width: 800px; margin: 0 auto; padding: 1rem; font-family: sans-serif; line-height: 1.6; }}
  .abstract {{ margin-bottom: 2rem; font-style: italic; }}
  math {{ display: inline-block; padding: 0 0.2rem; }}
</style>
</head>
{body}
</html>"""

    if "<body>" in html:
        body_content = html.split("<body>")[1].split("</body>")[0]
        html = html_template.format(body=f"<body>{body_content}</body>")
    else:
        html = html_template.format(body=f"<body>{html}</body>")
        
    return html

def main():
    papers_dir = "papers"
    for f in os.listdir(papers_dir):
        if f.endswith(".tex"):
            filepath = os.path.join(papers_dir, f)
            with open(filepath, "r") as file:
                content = file.read()
            html_content = tex_to_html(content)
            with open(os.path.join(papers_dir, f.replace(".tex", ".html")), "w") as file:
                file.write(html_content)

if __name__ == "__main__":
    main()
