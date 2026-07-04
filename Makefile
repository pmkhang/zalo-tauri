SHELL := /bin/bash

APP := zalo-tauri
PREFIX ?= $(HOME)/.local
MANIFEST := src-tauri/Cargo.toml
RELEASE_BIN := src-tauri/target/release/$(APP)
INSTALL_BIN := $(PREFIX)/bin/$(APP)
APPLICATIONS_DIR := $(PREFIX)/share/applications
ICONS_DIR := $(PREFIX)/share/icons/hicolor/128x128/apps
AUTOSTART_DIR := $(HOME)/.config/autostart
UID := $(shell id -u)
RUNTIME_DIR ?= /run/user/$(UID)
USER_ENV := env XDG_RUNTIME_DIR=$(RUNTIME_DIR) DBUS_SESSION_BUS_ADDRESS=unix:path=$(RUNTIME_DIR)/bus

.DEFAULT_GOAL := help

.PHONY: help check fmt build dev install start stop restart status logs clean uninstall

help:
	@printf '%s\n' \
	  'make check      - kiểm tra mã nguồn' \
	  'make fmt        - format mã Rust' \
	  'make build      - build binary release' \
	  'make dev        - chạy trực tiếp bằng cargo' \
	  'make install    - build và cài vào ~/.local' \
	  'make start      - mở app hoặc focus instance đang chạy' \
	  'make stop       - dừng app hoàn toàn' \
	  'make restart    - cài bản mới và khởi động lại' \
	  'make status     - xem trạng thái app' \
	  'make logs       - theo dõi log runtime' \
	  'make clean      - xóa output build' \
	  'make uninstall  - gỡ app, launcher và autostart'

check:
	cargo check --manifest-path $(MANIFEST)

fmt:
	cargo fmt --manifest-path $(MANIFEST)

build:
	cargo build --release --locked --manifest-path $(MANIFEST)

dev:
	cargo run --manifest-path $(MANIFEST)

install: build
	install -Dm755 $(RELEASE_BIN) $(INSTALL_BIN)
	install -Dm644 src-tauri/icons/128x128.png $(ICONS_DIR)/$(APP).png
	@mkdir -p $(APPLICATIONS_DIR) $(AUTOSTART_DIR)
	sed 's|@BINDIR@|$(PREFIX)/bin|g' packaging/$(APP).desktop.in > $(APPLICATIONS_DIR)/$(APP).desktop
	sed 's|@BINDIR@|$(PREFIX)/bin|g' packaging/$(APP)-autostart.desktop.in > $(AUTOSTART_DIR)/$(APP).desktop
	@command -v update-desktop-database >/dev/null && update-desktop-database $(APPLICATIONS_DIR) || true
	@command -v gtk-update-icon-cache >/dev/null && gtk-update-icon-cache -f $(PREFIX)/share/icons/hicolor >/dev/null || true
	@echo "Đã cài: $(INSTALL_BIN)"

start:
	@if pgrep -x $(APP) >/dev/null; then \
		$(INSTALL_BIN); \
	else \
		$(USER_ENV) systemd-run --user --unit=$(APP) --collect $(INSTALL_BIN); \
	fi

stop:
	@$(USER_ENV) systemctl --user stop $(APP).service 2>/dev/null || true
	@pkill -x $(APP) 2>/dev/null || true

restart: install stop start

status:
	@$(USER_ENV) systemctl --user --no-pager --full status $(APP).service 2>/dev/null || true
	@pgrep -a -x $(APP) || echo 'Zalo hiện không chạy.'

logs:
	$(USER_ENV) journalctl --user -u $(APP).service -f

clean:
	cargo clean --manifest-path $(MANIFEST)

uninstall: stop
	rm -f $(INSTALL_BIN)
	rm -f $(APPLICATIONS_DIR)/$(APP).desktop
	rm -f $(ICONS_DIR)/$(APP).png
	rm -f $(AUTOSTART_DIR)/$(APP).desktop
	@command -v update-desktop-database >/dev/null && update-desktop-database $(APPLICATIONS_DIR) || true
	@echo 'Đã gỡ Zalo Tauri. Dữ liệu đăng nhập được giữ nguyên.'
