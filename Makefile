
NAME=convnum
PREFIX=/usr/local

all:
	cargo build --release

install:
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f target/release/${NAME} ${DESTDIR}${PREFIX}/bin
	chmod +x ${DESTDIR}${PREFIX}/bin/${NAME}

# Copy the .desktop file so that app launchers can find it
xdg:
	cp ${NAME}.desktop ${DESTDIR}${PREFIX}/share/applications/

.PHONY: all install xdg

