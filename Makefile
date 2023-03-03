.PHONY: download scaffold

all: download scaffold

download:
	@for i in {01..25}; do \
	    cargo download $$i -y 2021; \
	done

scaffold:
	@for i in {01..25}; do \
	    cargo scaffold $$i -y 2021; \
	done
