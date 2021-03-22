BUILD = build
WEB = web

all: rust web

web: FORCE
	cp -r $(WEB)/* $(BUILD)

rust: dir FORCE
	wasm-pack build --target web -d $(BUILD)/pkg

dir:
	mkdir -p $(BUILD)

FORCE: ;

clean:
	rm -r $(BUILD)
