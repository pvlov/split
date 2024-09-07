.PHONY: models docs compose recompose clean purge


# OPENAPI 

OPENAPI_GENERATOR=openapi-generator-cli generate
OPENAPI_MODELS_CONFIG_FILE=openapi-config.yaml
OPENAPI_MODELS_OUT=target/openapi/
OPENAPI_DOCS_OUT=documentation/

all:
	# @make models
	@make compose

models:
	@$(OPENAPI_GENERATOR) -i openapi/user.yaml -g rust -c $(OPENAPI_MODELS_CONFIG_FILE) -o $(OPENAPI_MODELS_OUT) 

docs:
	@$(OPENAPI_GENERATOR) -i openapi/user.yaml -g html2 -o $(OPENAPI_DOCS_OUT) 

compose:
	@docker compose --profile backend up -d
	@yes | docker image prune --filter label=stage=builder

recompose: 
	@docker compose up --build server -d
	@yes | docker image prune --filter label=stage=builder # removes all builder stages used 

clean:
	@cargo clean
	@yes | docker image prune --filter label=stage=final

purge: 
	make clean
	@yes | docker volume rm $(docker volume ls -q --filter "label=stage=final") || echo "No volumes to clean up."
	
