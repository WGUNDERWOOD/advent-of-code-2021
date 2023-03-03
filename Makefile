.PHONY: download scaffold test run

default: run

all: download scaffold test run

download:
	for i in {01..25}; do \
	    cargo download $$i -y 2021; \
	done

scaffold:
	for i in {01..25}; do \
	    cargo scaffold $$i -y 2021; \
	done

test:
	cargo t

run:
	cargo run -r
