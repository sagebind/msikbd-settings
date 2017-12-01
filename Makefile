DESTDIR = /
PREFIX = $(DESTDIR)/usr/local
PACKAGE_NAME = msikbd-settings


all: target/release/$(PACKAGE_NAME)

target/release/$(PACKAGE_NAME):
	cargo build --release

install: target/release/$(PACKAGE_NAME)
	@echo Installing to $(PREFIX)/bin...
	install -m 0755 -D target/release/$(PACKAGE_NAME) $(PREFIX)/bin/$(PACKAGE_NAME)
	install -m 0644 -D extra/$(PACKAGE_NAME).desktop $(PREFIX)/share/applications/$(PACKAGE_NAME).desktop
	install -m 0644 -D extra/$(PACKAGE_NAME).service $(DESTDIR)/etc/systemd/system/$(PACKAGE_NAME).service

uninstall:
	@echo Uninstalling from $(PREFIX)/bin...
	rm $(PREFIX)/bin/$(PACKAGE_NAME)
	rm $(PREFIX)/share/applications/$(PACKAGE_NAME).desktop

clean:
	-rm -rf target
