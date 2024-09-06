#!/usr/bin/env bash

SERVICE=druidflow
SHELL=/bin/bash

docker compose up -d
docker compose exec $SERVICE $SHELL