# Utility script used to regenerate documentation.
.PHONY: docgen
docgen:
	(cd docgen && cargo run)
