.PHONY: all deploy openapi-models openapi-docs

OPENAPI_GENERATOR=openapi-generator-cli generate
OPENAPI_MODELS_CONFIG_PATH=openapi-config.yaml
OPENAPI_MODELS_OUT_DIR=target/openapi/
OPENAPI_DOCS_OUT_DIR=documentation/

all:
	make openapi
	make deploy

openapi-models:
	@$(OPENAPI_GENERATOR) -i openapi/user.yaml -g rust -c $(OPENAPI_MODELS_CONFIG_PATH) -o $(OPENAPI_MODELS_OUT_DIR) 

openapi-docs:
	@$(OPENAPI_GENERATOR) -i openapi/user.yaml -g html2 -o $(OPENAPI_DOCS_OUT_DIR) 

deploy:
	@docker compose up -d
	@docker image prune --filter label=stage=builder
