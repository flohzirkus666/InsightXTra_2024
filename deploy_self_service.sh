#!/bin/bash

# Install Docker
yum install -y docker podman-compose

# Verify Docker installation
docker --version

cd

cd Insight-XTra_2024-main/vue-frontend
docker build -t vue-frontend:v1 .

cd ../python-rest_endpoint
docker build -t python-api:v1 .

cd ../rust-rest_endpoint
docker build -t rust-api:v1 .