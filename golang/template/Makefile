PROJECT_NAME = template
PROJECT_PATH = github.com/yeluyang/playground/golang/$(PROJECT_NAME)
DOCKER_GOPATH = /go

.PHONY: all
all:

.PHONY: _pb
_pb:
	protoc --go_out=${GOPATH}/src \
		-I=${GOPATH}/src \
		$(shell pwd)/pb/api.proto

.PHONY: pb
pb:
	docker run -it --rm --name $(PROJECT_NAME)-env-pb \
		-e PROTOC_INSTALL=/go \
		-v $(shell pwd):$(DOCKER_GOPATH)/src/$(PROJECT_PATH) \
		-w $(DOCKER_GOPATH)/src/$(PROJECT_PATH) \
		brennovich/protobuf-tools make _pb
