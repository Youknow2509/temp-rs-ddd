# ===
# Variables
# ===
PROJECT_NAME := template-rust-server
CARGO := cargo
BIN   := cli

PATH_RUST_DOCS := docs/rustdoc
PATH_DOCKER_ENV_CP_DEV := environment/docker-compose-dev.yml
PATH_DOCKER_ENV_FILE := environment/.env
PATH_DOCKER_ENV_FILE_CP := environment/.env.dev
PATH_DOCKER_ENV_FILE_GEN_CONFIG_S3 := environment/garage/create_garage_config_dev.sh

# ===
# Targets
# ===
.DEFAULT_GOAL := help
.PHONY: help dev run build check fmt lint test clean \
        dev-cron run-cron \
        db-up db-down db-migrate db-reset \
        docker-build setup \
		docs

# ===
# Docker
# ===
docker-create-env: ## Create docker environment for development
	@echo "Creating .env file for docker environment..."
	@cp $(PATH_DOCKER_ENV_FILE_CP) $(PATH_DOCKER_ENV_FILE)

	@echo "Generating S3 config for docker environment..."
	@chmod +x $(PATH_DOCKER_ENV_FILE_GEN_CONFIG_S3)
	@$(PATH_DOCKER_ENV_FILE_GEN_CONFIG_S3)

docker-run-env: ## Run enviroment for server
	@echo "Starting docker environment for development..."
	docker compose -p $(PROJECT_NAME) -f $(PATH_DOCKER_ENV_CP_DEV) --env-file $(PATH_DOCKER_ENV_FILE) up -d

docker-temp-keyspace: ## Create template keyspace in Scylladb
	@echo "CREATE KEYSPACE IF NOT EXISTS main_keyspace WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': 1};"

# ===
# Docs
# ===
docs: ## Generate rustdoc documentation
	$(CARGO) doc --workspace --no-deps --document-private-items --target-dir $(PATH_RUST_DOCS)

# ===
# Development
# ===

dev: ## Run server with hot reload (APP_MODE=development)
	RUST_BACKTRACE=1 APP_MODE=development $(CARGO) watch -c -x "run -p $(BIN) -- serve"

run: ## Run server (uses default APP_MODE=production)
	$(CARGO) run -p $(BIN) -- serve

dev-cron: ## Run cronjob with hot reload (APP_MODE=development)
	APP_MODE=development $(CARGO) watch -c -x "run -p $(BIN) -- cronjob"

run-cron: ## Run cronjob (uses default APP_MODE=production)
	$(CARGO) run -p $(BIN) -- cronjob

build: ## Build release binary
	$(CARGO) build -p $(BIN) --release

# ===
# Code quality
# ===

check: ## Fast compile check
	$(CARGO) check --workspace --all-targets

fmt: ## Format code
	$(CARGO) fmt --all

lint: ## Run clippy
	$(CARGO) clippy --workspace --all-targets -- -D warnings

test: ## Run all tests
	$(CARGO) test --workspace

# ===
# Setup
# ===

setup: ## Install required cargo tools
	cargo install cargo-watch

clean: ## Remove build artifacts
	$(CARGO) clean

# ===
# Help
# ===

help: ## Show all commands
	@echo "Usage: make [target]"
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'