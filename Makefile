.PHONY: all deploy openapi-models openapi-docs


all:
	make openapi
	make deploy

openapi-models:
	@openapi-generator-cli generate -i openapi/user.yaml -g rust -c openapi-config.yaml -o target/openapi/ 

openapi-docs:
	@openapi-generator-cli generate -i openapi/user.yaml -g html2 -o documentation/

deploy:
	@docker compose up -d
	@docker image prune --filter label=stage=builder
