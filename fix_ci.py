import sys

content = open('/app/.github/workflows/ci.yml').read()

import re

# Resolve the matrix conflict
content = re.sub(
r"""<<<<<<< HEAD
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            name: Windows Native
          - os: macos-latest
            target: x86_64-apple-darwin
            name: macOS Native
          - os: ubuntu-latest
            target: wasm32-unknown-unknown
=======
            os: ubuntu-latest
            artifact_name: linux_artifact
          - target: x86_64-pc-windows-msvc
            name: Windows Native
            os: windows-latest
            artifact_name: windows_installer
          - target: x86_64-apple-darwin
            name: macOS Native
            os: macos-latest
            artifact_name: macos_app
          - target: wasm32-unknown-unknown
>>>>>>> 12e366a .*""",
"""            artifact_name: linux_artifact
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            name: Windows Native
            artifact_name: windows_installer
          - os: macos-latest
            target: x86_64-apple-darwin
            name: macOS Native
            artifact_name: macos_app
          - os: ubuntu-latest
            target: wasm32-unknown-unknown""", content, flags=re.MULTILINE)

# Resolve Free Disk Space
content = re.sub(
r"""<<<<<<< HEAD
        if: matrix.os == 'ubuntu-latest'
=======
        if: runner.os == 'Linux'
>>>>>>> 12e366a .*""",
"        if: runner.os == 'Linux'", content, flags=re.MULTILINE)

# Resolve Python/PyTorch setup (we remove it)
content = re.sub(
r"""<<<<<<< HEAD
=======
      - name: Set up Python.*?>>>>>>> 12e366a .*?\n""",
"", content, flags=re.DOTALL)

# Resolve verify-records
content = re.sub(
r"""<<<<<<< HEAD
      - name: Verify Architectural Records Updates
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p xtask -- verify-records
=======
      - name: Build Unified Verification Tool
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo build -p unified_verification --release

      - name: Verify Architectural Records Updates
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p unified_verification --release -- verify-records
>>>>>>> 12e366a .*?\n""",
"""      - name: Build Unified Verification Tool
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo build -p unified_verification --release

      - name: Verify Architectural Records Updates
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p unified_verification --release -- verify-records\n""", content, flags=re.MULTILINE)


# Resolve Check Traceability
content = re.sub(
r"""<<<<<<< HEAD
        run: cargo run -p xtask -- traceability
=======
        run: cargo run -p unified_verification --release -- traceability
>>>>>>> 12e366a .*?\n""",
"""        run: cargo run -p unified_verification --release -- traceability\n""", content, flags=re.MULTILINE)

# Resolve Check File Lengths
content = re.sub(
r"""<<<<<<< HEAD
=======
      - name: Check File Lengths \(500 lines limit\)
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p unified_verification --release -- check-file-lengths

>>>>>>> 12e366a .*?\n""",
"""      - name: Check File Lengths (500 lines limit)
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p unified_verification --release -- check-file-lengths\n\n""", content, flags=re.MULTILINE)

# Resolve Test Run
content = re.sub(
r"""<<<<<<< HEAD
        run: cargo test --workspace --verbose --all-features
=======
        shell: bash
        run: \|
          if \[ "\$RUNNER_OS" == "Windows" \]; then
            export PATH=\$\(python -c "import torch; print\(torch.__path__\[0\] \+ '/lib'\)"\):\$PATH
          elif \[ "\$RUNNER_OS" == "macOS" \]; then
            export DYLD_LIBRARY_PATH=\$\(python3 -c "import torch; print\(torch.__path__\[0\] \+ '/lib'\)"\):\$DYLD_LIBRARY_PATH
          else
            export LD_LIBRARY_PATH=\$\(python3 -c "import torch; print\(torch.__path__\[0\] \+ '/lib'\)"\):\$LD_LIBRARY_PATH
          fi
          cargo test --workspace --verbose --all-features
        env:
          LIBTORCH_USE_PYTORCH: 1
          LIBTORCH_BYPASS_VERSION_CHECK: 1
>>>>>>> 12e366a .*?\n""",
"""        run: cargo test --workspace --verbose --all-features\n""", content, flags=re.MULTILINE)


# Resolve verify suite
content = re.sub(
r"""<<<<<<< HEAD
      - name: Run Integrity Verified Suite
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p xtask -- verify-suite
=======
        env:
          LIBTORCH_USE_PYTORCH: 1
          LIBTORCH_BYPASS_VERSION_CHECK: 1

      - name: Install cargo-llvm-cov
        if: matrix.target != 'wasm32-unknown-unknown'
        uses: taiki-e/install-action@cargo-llvm-cov

      - name: Free up disk space before coverage
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo clean

      - name: Run Integrity Verified Suite
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p unified_verification --release -- verify-suite
        env:
          LIBTORCH_USE_PYTORCH: 1
          LIBTORCH_BYPASS_VERSION_CHECK: 1
>>>>>>> 12e366a .*?\n""",
"""      - name: Install cargo-llvm-cov
        if: matrix.target != 'wasm32-unknown-unknown'
        uses: taiki-e/install-action@cargo-llvm-cov

      - name: Free up disk space before coverage
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo clean

      - name: Run Integrity Verified Suite
        if: matrix.target != 'wasm32-unknown-unknown'
        run: cargo run -p unified_verification --release -- verify-suite\n""", content, flags=re.MULTILINE)


open('/app/.github/workflows/ci.yml', 'w').write(content)
