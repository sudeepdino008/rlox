CARGO = cargo


build:
	$(CARGO) build

run:
	$(CARGO) run

runf:
	$(CARGO) run -- $(file)

test:
	$(CARGO) test --workspace

testv:
	$(CARGO) test --workspace -- --show-output

clean:
	$(CARGO) clean

linter:
	$(CARGO) clippy --fix --workspace
