#!/bin/bash
set -e

# Setup PyTorch path for AI module if testing locally
if python3 -c "import torch" 2>/dev/null; then
    export LD_LIBRARY_PATH=$(python3 -c "import torch; print(torch.__path__[0] + '/lib')"):$LD_LIBRARY_PATH
    export LIBTORCH_USE_PYTORCH=1
    export LIBTORCH_BYPASS_VERSION_CHECK=1
else
    echo "Warning: PyTorch not found. AI tests may fail if they rely on native PyTorch bindings."
fi

FEATURES=("pure_math" "applied" "ai" "biology" "climate" "epidemiology" "physics")

echo "=== Running core-only (no features) ==="
cargo test -p math_explorer --no-default-features

for feature in "${FEATURES[@]}"; do
    echo "=== Running tests for feature: $feature ==="
    cargo test -p math_explorer --no-default-features --features "$feature"
done

echo "=== Running all features ==="
cargo test -p math_explorer --all-features

echo "All feature combinations passed successfully!"
