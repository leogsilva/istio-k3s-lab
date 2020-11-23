#!/bin/bash

pushd ${PROJECT_HOME}/microservices/webservice
sudo docker build -t webserver:v1.0.0 -f Dockerfile .
sudo docker save --output webserver_v1.0.0.tar webserver:v1.0.0
sudo /usr/local/bin/k3s ctr images import /home/ec2-user/istio-k3s-lab/microservices/webservice/webserver_v1.0.0.tar
popd

pushd ${PROJECT_HOME}/microservices/mock-security-service
sudo docker build -t mock-security-service:v1.0.0 -f Dockerfile .
sudo docker save --output mock-security-service_v1.0.0.tar mock-security-service:v1.0.0
sudo /usr/local/bin/k3s ctr images import /home/ec2-user/istio-k3s-lab/microservices/mock-security-service/mock-security-service_v1.0.0.tar
popd
