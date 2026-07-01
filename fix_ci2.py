import sys, re
content = open('/app/.github/workflows/ci.yml').read()
content = re.sub(
r"""<<<<<<< HEAD\n\n      - name: Run Integrity Verified Suite.*?>>>>>>> 12e366a .*?\n""",
"""      - name: Install cargo-llvm-cov
        if: matrix.target != 'wasm32-unknown-unknown'
        uses: taiki-e/install-action@cargo-llvm-cov

      - name: Free up disk space before coverage
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo clean

      - name: Run Integrity Verified Suite
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p unified_verification --release -- verify-suite\n""", content, flags=re.DOTALL)
open('/app/.github/workflows/ci.yml', 'w').write(content)
