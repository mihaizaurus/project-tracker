# Windows:
# set shell := ["powershell.exe","-c"]

# MacOs

run-db:
    cargo run -p project-tracker-db

run-back:
    cargo run -p project-tracker-backend

run-cli:
    cargo run -p project-tracker-cli