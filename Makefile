.PHONY: help

CYAN := \033[36m
RESET := \033[0m
SPACE_WIDTH := 20

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | while IFS=':' read -r target description; do \
		description=$$(echo $$description | sed 's/^.*##//'); \
		printf "  $(CYAN)%-$(SPACE_WIDTH)s$(RESET) %s\n" "make $$target" "$$description"; \
	done

.PHONY: build_dev
build_dev: ## Build server and client for development
	@echo "Building server and client for development"
	cargo build
	pnpm compile

.PHONY: build_release
build_release: ## Build server and client for release
	@echo "Building server and client for release"
	cargo build
	pnpm compile
	vsce package -o dist/

test: ## Run tests
	@echo "Running tests"
	cargo test

.PHONY: build_antlr
build_antlr: ## Build ANTLR
	@echo "Building ANTLR"
	@cd ./assets/antlr/ && java -cp ./antlr4-4.8-2-SNAPSHOT-complete.jar org.antlr.v4.Tool -Dlanguage=Rust -o ../../src/antlr/generated PainlessParser.g4  -visitor
	@cd ./assets/antlr/ && java -cp ./antlr4-4.8-2-SNAPSHOT-complete.jar org.antlr.v4.Tool -Dlanguage=Rust -o ../../src/antlr/generated PainlessLexer.g4  -visitor
