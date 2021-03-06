#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

echo '''
#!/bin/bash

set -e

cd "$(git rev-parse --show-toplevel)"
staged_rust=$(git diff --staged --name-only --diff-filter=d | grep -e '.rs$') || exit 0

rustfmt $staged_rust
git add $staged_rust

if ! make check; then
    echo "Failed check."
    exit 1
fi
''' > .git/hooks/pre-commit

chmod +x .git/hooks/pre-commit
