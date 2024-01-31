#!/bin/bash
GIT=$(git rev-parse --short HEAD)
echo $GIT
docker buildx build --build-arg GIT=$GIT --platform linux/amd64 -t apwells/univeral-bottle-tracker:$GIT -t apwells/univeral-bottle-tracker:latest .
docker push -a apwells/univeral-bottle-tracker
