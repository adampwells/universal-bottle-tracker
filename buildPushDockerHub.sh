#!/bin/bash
GIT=$(git rev-parse --short HEAD)
echo $GIT
docker buildx build --build-arg GIT=$GIT --platform linux/amd64 -t apwells/universal-bottle-tracker:$GIT -t apwells/universal-bottle-tracker:latest .
docker push -a apwells/universal-bottle-tracker
