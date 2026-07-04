SHELL := /bin/bash

APP := zalo-tauri
PREFIX ?= $(HOME)/.local
MANIFEST := src-tauri/Cargo.toml
RELEASE_BIN := src-tauri/target/release/$(APP)
INSTALL_BIN := $(PREFIX)/bin/$(APP)
APPLICATIONS_DIR := $(PREFIX)/share/applications
ICONS_DIR := $(PREFIX)/share/icons/hicolor/128x128/apps
AUTOSTART_DIR := $(HOME)/.config/autostart
STATE_DIR := $(HOME)/.local/state/$(APP)
LOG_FILE := $(STATE_DIR)/$(APP).log
USER_ID := $(shell id -u)
RUNTIME_DIR ?= /run/user/$(USER_ID)
USER_ENV := env XDG_RUNTIME_DIR=$(RUNTIME_DIR) DBUS_SESSION_BUS_ADDRESS=unix:path=$(RUNTIME_DIR)/bus

.DEFAULT_GOAL := help
.NOTPARALLEL:

.PHONY: help deps doctor check fmt build dev install install-files start stop restart status logs clean uninstall

help:
	@printf '%s\n' \
	  'make deps       - cài build dependencies theo distro' \
	  'make doctor     - kiểm tra toolchain và thư viện hệ thống' \
	  'make check      - kiểm tra compile mã nguồn' \
	  'make fmt        - format mã Rust' \
	  'make build      - build binary release' \
	  'make dev        - chạy trực tiếp bằng cargo' \
	  'make install    - build và cài vào PREFIX (mặc định ~/.local)' \
	  'make start      - mở app hoặc focus instance đang chạy' \
	  'make stop       - dừng app hoàn toàn' \
	  'make restart    - cài bản mới và khởi động lại' \
	  'make status     - xem trạng thái app' \
	  'make logs       - theo dõi log runtime' \
	  'make clean      - xóa output build' \
	  'make uninstall  - gỡ app, launcher và autostart'

deps:
	@if command -v pacman >/dev/null 2>&1; then \
		sudo pacman -Syu; \
		sudo pacman -S --needed base-devel curl wget file openssl gtk3 webkit2gtk-4.1 libayatana-appindicator librsvg xdotool gst-plugins-good; \
	elif command -v apt-get >/dev/null 2>&1; then \
		sudo apt-get update; \
		sudo apt-get install -y build-essential curl wget file libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev gstreamer1.0-plugins-good; \
	elif command -v dnf >/dev/null 2>&1; then \
		sudo dnf group install -y 'c-development'; \
		sudo dnf install -y webkit2gtk4.1-devel gtk3-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel libxdo-devel gstreamer1-plugins-good; \
	elif command -v zypper >/dev/null 2>&1; then \
		sudo zypper --non-interactive install -t pattern devel_basis; \
		sudo zypper --non-interactive install webkit2gtk3-devel gtk3-devel libopenssl-devel curl wget file libappindicator3-1 librsvg-devel gstreamer-plugins-good; \
	else \
		echo 'Distro chưa được tự động nhận diện.' >&2; \
		echo 'Xem mục Yêu cầu hệ thống trong README.md.' >&2; \
		exit 1; \
	fi

doctor:
	@failed=0; \
	for command_name in cargo rustc make pkg-config; do \
		if command -v "$$command_name" >/dev/null 2>&1; then \
			printf 'OK   %s\n' "$$command_name"; \
		else \
			printf 'FAIL %s\n' "$$command_name"; \
			failed=1; \
		fi; \
	done; \
	for module in gtk+-3.0 webkit2gtk-4.1; do \
		if pkg-config --exists "$$module" 2>/dev/null; then \
			printf 'OK   %s %s\n' "$$module" "$$(pkg-config --modversion "$$module")"; \
		else \
			printf 'FAIL %s\n' "$$module"; \
			failed=1; \
		fi; \
	done; \
	if command -v gst-inspect-1.0 >/dev/null 2>&1 && gst-inspect-1.0 autoaudiosink >/dev/null 2>&1; then \
		printf 'OK   GStreamer autoaudiosink\n'; \
	else \
		printf 'FAIL GStreamer autoaudiosink (cài plugins-good)\n'; \
		failed=1; \
	fi; \
	exit $$failed

check:
	cargo check --locked --manifest-path "$(MANIFEST)"

fmt:
	cargo fmt --manifest-path "$(MANIFEST)"

build:
	cargo build --release --locked --manifest-path "$(MANIFEST)"

dev:
	cargo run --manifest-path "$(MANIFEST)"

install: build install-files

install-files:
	install -Dm755 "$(RELEASE_BIN)" "$(INSTALL_BIN)"
	install -Dm644 src-tauri/icons/128x128.png "$(ICONS_DIR)/$(APP).png"
	@mkdir -p "$(APPLICATIONS_DIR)" "$(AUTOSTART_DIR)" "$(STATE_DIR)"
	sed 's|@BINDIR@|$(PREFIX)/bin|g' packaging/$(APP).desktop.in > "$(APPLICATIONS_DIR)/$(APP).desktop"
	sed 's|@BINDIR@|$(PREFIX)/bin|g' packaging/$(APP)-autostart.desktop.in > "$(AUTOSTART_DIR)/$(APP).desktop"
	@command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$(APPLICATIONS_DIR)" || true
	@command -v gtk-update-icon-cache >/dev/null 2>&1 && gtk-update-icon-cache -f "$(PREFIX)/share/icons/hicolor" >/dev/null || true
	@echo "Đã cài: $(INSTALL_BIN)"

start:
	@test -x "$(INSTALL_BIN)" || { echo 'Chưa cài app. Chạy make install trước.' >&2; exit 1; }
	@mkdir -p "$(STATE_DIR)"
	@if pgrep -x "$(APP)" >/dev/null; then \
		"$(INSTALL_BIN)"; \
	elif command -v systemd-run >/dev/null 2>&1 && [ -S "$(RUNTIME_DIR)/bus" ]; then \
		$(USER_ENV) systemd-run --user --unit=$(APP) --collect "$(INSTALL_BIN)"; \
	else \
		nohup "$(INSTALL_BIN)" >> "$(LOG_FILE)" 2>&1 & \
		echo "Đã khởi động $(APP) (PID $$!)."; \
	fi

stop:
	@if command -v systemctl >/dev/null 2>&1 && [ -S "$(RUNTIME_DIR)/bus" ]; then \
		$(USER_ENV) systemctl --user stop $(APP).service 2>/dev/null || true; \
	fi
	@pkill -x "$(APP)" 2>/dev/null || true

restart: install stop start

status:
	@if command -v systemctl >/dev/null 2>&1 && [ -S "$(RUNTIME_DIR)/bus" ]; then \
		$(USER_ENV) systemctl --user --no-pager --full status $(APP).service 2>/dev/null || true; \
	fi
	@pgrep -a -x "$(APP)" || echo 'Zalo hiện không chạy.'

logs:
	@if command -v journalctl >/dev/null 2>&1 && [ -S "$(RUNTIME_DIR)/bus" ] && \
		$(USER_ENV) systemctl --user is-active --quiet $(APP).service 2>/dev/null; then \
		$(USER_ENV) journalctl --user -u $(APP).service -f; \
	else \
		mkdir -p "$(STATE_DIR)"; \
		touch "$(LOG_FILE)"; \
		tail -f "$(LOG_FILE)"; \
	fi

clean:
	cargo clean --manifest-path "$(MANIFEST)"

uninstall: stop
	rm -f "$(INSTALL_BIN)"
	rm -f "$(APPLICATIONS_DIR)/$(APP).desktop"
	rm -f "$(ICONS_DIR)/$(APP).png"
	rm -f "$(AUTOSTART_DIR)/$(APP).desktop"
	@command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$(APPLICATIONS_DIR)" || true
	@echo 'Đã gỡ Zalo Tauri. Dữ liệu đăng nhập và log được giữ nguyên.'
