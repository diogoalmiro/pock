
.PHONY: all

build/%.html: src/%.elm
	npx --yes elm@latest make $< --output=$@

all: build/Index.html build/World.html