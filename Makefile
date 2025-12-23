.PHONY: run bundle-linux

run:
	dx serve --release

bundle-linux:
	dx bundle --release --linux
