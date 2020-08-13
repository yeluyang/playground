PKGS = $(shell go list ./...|grep -v /vendor/)

.PHONY: all
all: direct combine

.PHONY: direct
direct:
	@mkdir -p tmp/direct
	@gocov test -timeout=20m -tags nolic -race -v $(PKGS) | python3 -m json.tool | tee tmp/direct/all.json | gocov-html > tmp/direct/coverage.html
	@for pkg in $(PKGS) ; do \
		echo $$pkg; \
		gocov test -timeout=20m -tags nolic -race -v $$pkg | python3 -m json.tool > tmp/direct/$$(basename $$pkg).json; \
    done

.PHONY: combine
combine:
	@mkdir -p tmp/combine
	@echo "mode: count" > tmp/combine/all.cov
	@for pkg in $(PKGS) ; do \
		go test -covermode=count -coverprofile=tmp/combine/$$(basename $$pkg).cov $$pkg; \
		gocov convert tmp/combine/$$(basename $$pkg).cov | python3 -m json.tool > tmp/combine/$$(basename $$pkg).json; \
		tail -n +2 tmp/combine/$$(basename $$pkg).cov >> tmp/combine/all.cov; \
    done
	@gocov convert tmp/combine/all.cov | python3 -m json.tool | tee tmp/combine/all.json | gocov-html > tmp/combine/coverage.html
