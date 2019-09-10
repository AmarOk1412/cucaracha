.PHONY: all

clean:
	cross clean --target arm-unknown-linux-gnueabihf

build:
	cross build --target arm-unknown-linux-gnueabihf

send:
	echo `du -hs target/arm-unknown-linux-gnueabihf/debug/cucaracha`
	scp target/arm-unknown-linux-gnueabihf/debug/cucaracha  debian@192.168.7.2:~/

all: build send