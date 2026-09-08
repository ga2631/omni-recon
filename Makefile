# ==============================================================================
# OmniRecon Project Makefile
# ==============================================================================

.PHONY: help dev dev-down dev-logs prod prod-build prod-down clean

help:
	@echo "OmniRecon Command Shortcuts:"
	@echo "  make dev         - Start development environment with Hot-Reload (Rust + Vue 3)"
	@echo "  make dev-down    - Stop development environment"
	@echo "  make dev-logs    - Follow development logs"
	@echo "  make prod        - Start production environment"
	@echo "  make prod-down   - Stop production environment"

# Development Mode (Hot-Reloading with Vite HMR & Cargo Watch)
dev:
	docker compose -f docker-compose.dev.yml up --build

dev-down:
	docker compose -f docker-compose.dev.yml down

dev-logs:
	docker compose -f docker-compose.dev.yml logs -f

# Production Mode
prod:
	docker compose up --build -d

prod-down:
	docker compose down

clean:
	docker compose -f docker-compose.dev.yml down -v
	docker compose down -v
