#!/usr/bin/env bash
docker kill $(docker container ls -q)