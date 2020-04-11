prefix ?= /usr/local
bindir = $(prefix)/bin
appdir = $(prefix)/share/applications
icondir = $(prefix)/share/icons/hicolor/scalable/apps

RELEASE=target/release/
APP_ID=typeonscreen

all: $(APP_ID)

$(APP_ID):
	cargo build --release ; \

clean:
	cargo clean

distclean: clean
	rm -rf .cargo

install:
	install -D $(RELEASE)$(APP_ID) $(DESTDIR)$(bindir)/$(APP_ID)
	install -Dm644 resources/$(APP_ID).desktop $(DESTDIR)$(appdir)/$(APP_ID).desktop
	install -Dm644 resources/$(APP_ID).svg $(DESTDIR)$(icondir)/$(APP_ID).svg
	install -Dm644 resources/$(APP_ID)-enable.svg $(DESTDIR)$(icondir)/$(APP_ID)-enable.svg

uninstall:
	rm $(DESTDIR)$(bindir)/$(APP_ID)
	rm $(DESTDIR)$(appdir)/$(APP_ID).desktop
	rm $(DESTDIR)$(icondir)/$(APP_ID).svg
	rm $(DESTDIR)$(icondir)/$(APP_ID)-enable.svg