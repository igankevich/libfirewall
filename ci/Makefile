CI := ghcr.io/igankevich/libfirewall-ci:latest

.PHONY: ci
ci:
	docker build --tag $(CI) - < Dockerfile
	docker push $(CI)
