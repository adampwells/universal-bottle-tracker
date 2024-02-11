#!/bin/bash
quasar build
rm -rf ../static/*
mv dist/spa/* ../static/