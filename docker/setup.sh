#!/usr/bin/env bash

docker build --build-arg force_update=$(date '+%s') -t yew-app - < ./.devcontainer/Dockerfile
