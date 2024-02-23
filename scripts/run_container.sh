#!/usr/bin/env bash
set -eo pipefail

CONTAINER_ID=$(docker run --rm --env-file .env -p 3000:3000 -d pokemon-graphql)

docker logs "${CONTAINER_ID}" --follow | bunyan