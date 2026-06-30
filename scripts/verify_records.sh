#!/bin/bash
set -e

# Determine the commits to check for [skip journal]
if [ -n "$BASE_SHA" ] && [ -n "$HEAD_SHA" ]; then
    echo "Comparing $BASE_SHA to $HEAD_SHA..."
    # If the user put [skip journal] in ANY commit in the PR, we bypass.
    COMMIT_MSGS=$(git log --format=%B $BASE_SHA..$HEAD_SHA || git log -1 --pretty=%B)
    CHANGED_FILES=$(git diff --name-only $BASE_SHA $HEAD_SHA || git diff-tree --no-commit-id --name-only -r HEAD)
else
    echo "Checking latest commit..."
    COMMIT_MSGS=$(git log -1 --pretty=%B)
    CHANGED_FILES=$(git diff-tree --no-commit-id --name-only -r HEAD)
fi

if echo "$COMMIT_MSGS" | grep -iq "\[skip journal\]"; then
    echo "Commit message contains [skip journal]. Bypassing journal verification."
    exit 0
fi

echo "Changed files:"
echo "$CHANGED_FILES"

# Check if any core files were modified
if echo "$CHANGED_FILES" | grep -qE '^(math_explorer/|crates/)'; then
    echo "Core logic areas (math_explorer/ or crates/) were modified."
    # Check if records were updated
    if ! echo "$CHANGED_FILES" | grep -q '^\.jules/'; then
        echo "Error: Architectural records in .jules/ were not updated."
        echo "Please update the corresponding journals when modifying core architecture."
        echo "If this is a hotfix or non-architectural change, add '[skip journal]' to the commit message."
        exit 1
    else
        echo "Architectural records successfully verified."
    fi
else
    echo "No core logic areas modified. Skipping journal verification."
fi
