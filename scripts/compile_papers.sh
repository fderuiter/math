#!/bin/bash
set -e

# Make sure we have the required package
pip install latex2mathml --break-system-packages 2>/dev/null || true

cd papers
echo "Compiling papers to PDF (if pdflatex is available)..."
if command -v pdflatex &> /dev/null; then
    for f in *.tex; do
        pdflatex -interaction=nonstopmode "$f" >/dev/null 2>&1 || true
    done
else
    echo "Warning: pdflatex not found, skipping PDF generation."
fi
cd ..

echo "Generating HTML with MathML..."
python3 scripts/generate_html.py
echo "Paper compilation complete."
