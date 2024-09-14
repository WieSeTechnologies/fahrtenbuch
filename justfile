set shell := ["nu", "-c"]
set windows-shell := ["nu", "-c"]

default: start


start:
  docker compose pull
  docker compose up --build --force-recreate -d

stop:
  docker compose down

restart:
  just stop-dev
  just start-dev
