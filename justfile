Windows:
# set shell := ["powershell.exe","-c"]

db-crate := "project-tracker-db"
cli-crate := "project-tracker-cli"
backend-crate := "project-tracker-backend"
domain-crate := "project-tracker-core"

run-db:
    cargo run -p {{db-crate}}
test-db:
  cargo test -p {{db-crate}}

run-back:
    cargo run -p {{backend-crate}}
test-back:
  cargo test -p {{backend-crate}}

run-cli:
    cargo run -p {{cli-crate}}
test-cli:
  cargo test -p {{cli-crate}}

run-core:
    cargo run -p {{domain-crate}}
test-core:
  cargo test -p {{domain-crate}}


