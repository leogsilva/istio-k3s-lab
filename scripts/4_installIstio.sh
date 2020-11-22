#!/bin/bash

sudo KUBECONFIG=/etc/rancher/k3s/k3s.yaml ${ISTIO_HOME}/bin/istioctl install -f ${PROJECT_HOME}/istio-config/istio-config.yaml