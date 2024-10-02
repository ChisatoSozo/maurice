build:
	bash scripts/build.sh

dev:
	lsof -ti:8080 2>/dev/null | xargs kill -9 2>/dev/null || true
	cd server && PORT=9090 cargo run
	cd ..

prod:
	lsof -ti:8080 2>/dev/null | xargs kill -9 2>/dev/null || true
	cd server && PORT=8080 cargo run --release
	cd ..