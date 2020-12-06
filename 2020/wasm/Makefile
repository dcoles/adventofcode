SRCS := $(shell find src/ -type f -name '*.rs')

.PHONY: all clean

all: pkg/wasm.js

clean:
	-rm -r pkg

pkg/wasm.js: $(SRCS)
	wasm-pack build --release --target web
