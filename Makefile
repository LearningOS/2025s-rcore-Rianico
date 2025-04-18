DOCKER_NAME ?= rcore-tutorial-v3
.PHONY: docker build_docker
	
docker:
	podman run --rm -it -v ${PWD}:/mnt -w /mnt --user $(id -u):$(id -g) ${DOCKER_NAME} bash

build_docker: 
	podman build -t ${DOCKER_NAME} .

fmt:
	cd os ; cargo fmt;  cd ..

