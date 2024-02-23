#!/usr/bin/env bash
set -eo pipefail

if  [[ -z "$SKIP_ENV" ]]
then
  if [ -f .env ]
  then
    source .env
  else
    echo >&2 "Error: .env file not found"
    exit 1
  fi
fi


if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
  # if a postgres container is running, print instructions to kill it and exit
  RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=pokemon' --format '{{.ID}}')
  if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
    echo >&2 "there is a postgres container already running, kill it with"
    echo >&2 "    docker kill ${RUNNING_POSTGRES_CONTAINER}"
    exit 1
  fi
  # Launch postgres using Docker
  docker run \
      -e POSTGRES_USER="${DATABASE_USERNAME}" \
      -e POSTGRES_PASSWORD="${DATABASE_PASSWORD}" \
      -e POSTGRES_DB="${DATABASE_NAME}" \
      -p "${DATABASE_PORT}":5432 \
      -d \
      --name "pokemon_$(date '+%s')" \
      --rm \
      postgres -N 1000
      # ^ Increased maximum number of connections for testing purposes
fi

# Keep pinging Postgres until it's ready to accept commands
until PGPASSWORD="${DATABASE_PASSWORD}" psql -h "${DATABASE_HOST}" -U "${DATABASE_USERNAME}" -p "${DATABASE_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DATABASE_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DATABASE_USERNAME}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"