.PHONY: run down build up

run: down build up

down:
	docker-compose down -v

build:
	docker-compose build

up:
	docker-compose up

init_folders:
	@mkdir -p config
	@mkdir -p src
	@touch Cargo.toml
	@touch config/config.yaml
	@touch src/main.rs
	@touch src/config.rs
	@touch src/db.rs
	@touch src/model.rs
	@touch src/writer.rs
	@touch src/partition.rs
	@touch src/utils.rs
	@echo "Project structure initialized."
