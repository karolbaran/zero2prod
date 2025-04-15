#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed. Please install PostgreSQL client."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed. Please install PostgreSQL client."
  echo >&2 "Use:"
  echo >&2 "cargo install sqlx-cli"
  echo >&2 "to install."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a custom password has been set, otherwise default to 'postgres'
DB_PASSWORD=${POSTGRES_PASSWORD:=postgres}
# Check if a custom database has been set, otherwise default to 'newsletter'
DB_NAME=${POSTGRES_DB:=newsletter}
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST=${POSTGRES_HOST:=localhost}
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT=${POSTGRES_PORT:=5432}

# Launch postgres using Docker
docker run \
  -e POSTGRES_USER=$DB_USER \
  -e POSTGRES_PASSWORD=$DB_PASSWORD \
  -e POSTGRES_DB=$DB_NAME \
  -p $DB_PORT:5432 \
  --name newsletter-db \
  -d postgres:latest \
  postgres -N 1000

# Keep pinging the database until it is ready to accept commands
export PGPASSWORD=$DB_PASSWORD
until psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres is up and running on port ${DATABASE_URL}"
>&2 echo "Postgres has been migrated,"
