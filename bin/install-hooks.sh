#!/bin/bash

set -e

cd $(git rev-parse --show-toplevel)

cat << EOF > .git/hooks/pre-commit
#!/bin/bash

cd $(git rev-parse --show-toplevel)
rustfmt --check **/*.rs
cargo check
EOF

chmod +x .git/hooks/pre-commit
