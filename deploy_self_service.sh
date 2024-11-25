#!/bin/bash

# Update the yum package index
yum update -y

# Install Docker
yum install -y docker docker-compose



# Verify Docker installation
docker --version

cd

cd Insight-XTra_2024-main/vue-frontend
docker build -t vue-frontend:v1 .

cd ../python-rest_endpoint
docker build -t python-api:v1 .

cd ../rust-rest_endpoint
docker build -t rust-api:v1 .