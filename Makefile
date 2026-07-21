APP_NAME     := xdg-mime-gui
DESKTOP_FILE := io.github.tomas_barros1.DefaultApps.desktop
PREFIX       ?= /usr/local
BINDIR       ?= $(PREFIX)/bin
DATADIR      ?= $(PREFIX)/share
APPDIR       ?= $(DATADIR)/applications
ICONDIR      ?= $(DATADIR)/icons/hicolor/scalable/apps
CARGO        ?= cargo

RELEASE      := target/release/$(APP_NAME)

.PHONY: all build release clean install uninstall run check lint

all: release

build:
	$(CARGO) build

release: $(RELEASE)

$(RELEASE):
	$(CARGO) build --release

clean:
	$(CARGO) clean
	rm -rf target

install: $(RELEASE)
	install -Dm755 $(RELEASE) $(DESTDIR)$(BINDIR)/$(APP_NAME)
	install -Dm644 $(DESKTOP_FILE) $(DESTDIR)$(APPDIR)/$(DESKTOP_FILE)

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/$(APP_NAME)
	rm -f $(DESTDIR)$(APPDIR)/$(DESKTOP_FILE)

run: build
	$(CARGO) run

check:
	$(CARGO) check

lint:
	$(CARGO) fmt --check
	$(CARGO) clippy -- -D warnings
