build:
	bash scripts/build.sh

run:
	lsof -ti:8080 2>/dev/null | xargs kill -9 2>/dev/null || true
	cd server && cargo run
	cd ..