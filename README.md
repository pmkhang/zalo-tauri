# Zalo for Linux (Tauri)

Ứng dụng desktop Linux tối giản cho `https://chat.zalo.me/`, viết bằng Tauri v2 và WebKitGTK. Project không đóng gói hay sửa mã nguồn Zalo; nó chỉ tạo cửa sổ native, system tray và tích hợp notification cho Zalo Web.

## Tính năng

- Thanh title custom nhỏ gọn với nút thu nhỏ, phóng to và đóng.
- Kéo cửa sổ bằng vùng title; double-click để phóng to hoặc khôi phục.
- Đóng cửa sổ sẽ ẩn xuống system tray thay vì thoát.
- Menu tray gồm **Mở Zalo**, **Ẩn Zalo** và **Thoát hoàn toàn**.
- Chỉ cho phép một instance. Mở lần hai sẽ hiện và focus cửa sổ đang chạy.
- Chuyển Web Notification của Zalo thành notification native trên Linux.
- Tự khởi động sau khi đăng nhập desktop.
- Nội dung từ xa không được cấp Tauri command/capability.

## Yêu cầu hệ thống

Ứng dụng chạy trên desktop Linux dùng GTK 3 và WebKitGTK 4.1, hỗ trợ cả Wayland lẫn X11. Project hiện được kiểm thử trực tiếp trên Arch Linux + KDE Plasma + Wayland; các lệnh bên dưới bao phủ những họ distro Linux phổ biến.

### Arch Linux / Manjaro / EndeavourOS

```bash
sudo pacman -Syu
sudo pacman -S --needed \
  base-devel curl wget file openssl \
  gtk3 webkit2gtk-4.1 libayatana-appindicator librsvg xdotool
```

### Debian / Ubuntu / Linux Mint / Pop!_OS

```bash
sudo apt update
sudo apt install -y \
  build-essential curl wget file \
  libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev
```

Ubuntu 22.04 và Debian 12 là baseline phù hợp vì repository chuẩn đã cung cấp WebKitGTK 4.1.

### Fedora

```bash
sudo dnf check-update || true
sudo dnf group install -y "c-development"
sudo dnf install -y \
  webkit2gtk4.1-devel gtk3-devel openssl-devel \
  curl wget file libappindicator-gtk3-devel librsvg2-devel libxdo-devel
```

Trên Fedora Silverblue/Kinoite dùng cùng danh sách package với `rpm-ostree install`, sau đó reboot.

### openSUSE Tumbleweed / Leap

```bash
sudo zypper refresh
sudo zypper install -t pattern devel_basis
sudo zypper install \
  webkit2gtk3-devel gtk3-devel libopenssl-devel \
  curl wget file libappindicator3-1 librsvg-devel
```

### Rust stable

Cần `cargo` và `rustc`. Cách đồng nhất giữa các distro là cài Rust bằng `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default stable
```

Nếu distro đã cung cấp Rust đủ mới, có thể dùng package manager của distro thay cho `rustup`. Kiểm tra toolchain:

```bash
cargo --version
rustc --version
```

Danh sách dependencies dựa trên [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/). Tên package có thể thay đổi theo phiên bản distro; nếu package manager không tìm thấy một package, hãy tra package tương đương cung cấp `webkit2gtk-4.1.pc` hoặc `ayatana-appindicator3-0.1.pc`.

Có thể để Makefile tự nhận diện một trong bốn họ distro trên và cài dependencies:

```bash
make deps
make doctor
```

`make deps` sử dụng `sudo` và sẽ yêu cầu xác nhận quyền quản trị. `make doctor` chỉ kiểm tra, không thay đổi hệ thống.

## Build

Clone và đi vào thư mục project:

```bash
git clone https://github.com/pmkhang/zalo-tauri.git
cd zalo-tauri
```

Build release có tối ưu:

```bash
make build
```

Binary được tạo tại:

```text
src-tauri/target/release/zalo-tauri
```

Lần build đầu tiên có thể mất vài phút vì Rust phải biên dịch Tauri, GTK và WebKit bindings. Các lần sau dùng cache nên nhanh hơn.

Kiểm tra mã nguồn mà không tạo binary release:

```bash
make check
```

Format mã Rust:

```bash
make fmt
```

Chạy trực tiếp ở chế độ phát triển:

```bash
make dev
```

## Cài đặt cho người dùng hiện tại

```bash
make install
```

Lệnh trên sẽ:

1. Build binary release bằng `Cargo.lock`.
2. Cài binary vào `~/.local/bin/zalo-tauri`.
3. Cài icon vào `~/.local/share/icons`.
4. Thêm **Zalo** vào menu ứng dụng.
5. Thêm launcher tự khởi động vào `~/.config/autostart`.

Không cần `sudo` vì toàn bộ file được cài trong home directory.

Khởi động ứng dụng:

```bash
make start
```

Nếu Zalo đã chạy, lệnh này không tạo tiến trình thứ hai mà chỉ hiện và focus cửa sổ hiện tại.

## Cập nhật sau khi sửa mã nguồn

Build, cài binary mới và khởi động lại bằng một lệnh:

```bash
make restart
```

## Sử dụng

- Nút biểu tượng thu nhỏ: thu nhỏ cửa sổ.
- Nút phóng to: chuyển giữa trạng thái thường và toàn màn hình cửa sổ.
- Nút đóng: ẩn cửa sổ xuống system tray.
- Giữ và kéo vùng giữa title bar để di chuyển cửa sổ.
- Double-click vùng title bar để phóng to hoặc khôi phục.
- Chọn **Thoát hoàn toàn** trong menu tray để kết thúc tiến trình.

Thông báo tin nhắn dùng notification native của desktop. Trong Zalo Web vẫn cần bật thông báo tại **Cài đặt → Thông báo** và môi trường desktop không được chặn thông báo của ứng dụng.

## Quản lý tiến trình

Xem trạng thái:

```bash
make status
```

Dừng hoàn toàn:

```bash
make stop
```

Theo dõi log khi app được chạy qua `make start`:

```bash
make logs
```

## Gỡ cài đặt

```bash
make uninstall
```

Lệnh này gỡ binary, launcher, icon và autostart. Dữ liệu đăng nhập/WebKit được giữ lại để tránh xóa nhầm phiên người dùng.

## Dọn output build

```bash
make clean
```

Lệnh này xóa thư mục `target`; lần build tiếp theo sẽ phải biên dịch lại toàn bộ dependencies.

## Cấu trúc project

```text
zalo-tauri/
├── Makefile                       Lệnh build, cài đặt và quản lý app
├── README.md                      Tài liệu này
├── dist/                          Frontend fallback tối thiểu
├── packaging/                     Desktop entry và autostart templates
└── src-tauri/
    ├── Cargo.toml                 Rust dependencies
    ├── Cargo.lock                 Phiên bản dependency đã khóa
    ├── tauri.conf.json            Cấu hình cửa sổ và bundle
    ├── icons/                     Icon ứng dụng
    └── src/
        ├── main.rs                Entry point tối thiểu
        ├── app.rs                 Khởi tạo và ghép các thành phần Tauri
        ├── window.rs              Hiện, ẩn và xử lý đóng cửa sổ
        ├── tray.rs                System tray và menu
        └── linux/
            ├── mod.rs             Tích hợp riêng cho Linux
            ├── notifications.rs   Web notification và DBus native
            ├── restore_notification_permission.js
            │                      Khôi phục trạng thái quyền trước khi Zalo khởi tạo
            ├── titlebar.rs        Hành vi title bar GTK
            └── titlebar.css       Giao diện title bar
```

## Xử lý lỗi

### App không xuất hiện nhưng tiến trình vẫn chạy

Chọn **Mở Zalo** từ tray hoặc chạy:

```bash
make start
```

### Không có tray icon

KDE Plasma hỗ trợ tray sẵn. Với GNOME, có thể cần extension AppIndicator/KStatusNotifierItem. Đồng thời kiểm tra thư viện AppIndicator của distro đã được cài, rồi chạy:

```bash
make restart
```

### Không nhận được thông báo

1. Bật thông báo trong cài đặt Zalo Web.
2. Kiểm tra phần **Notifications** trong System Settings/Settings của desktop.
3. Giữ ứng dụng chạy dưới tray.
4. Xem log bằng `make logs`.

### Build lỗi do thiếu WebKitGTK

Cài lại dependencies theo đúng mục distro ở trên. Có thể xác minh development package bằng:

```bash
pkg-config --modversion webkit2gtk-4.1
pkg-config --modversion gtk+-3.0
make build
```

### Binary build trên distro này không chạy ở distro cũ hơn

Binary Linux phụ thuộc glibc của hệ thống dùng để build. Muốn phát hành rộng, hãy build trên baseline cũ nhất cần hỗ trợ, ví dụ Ubuntu 22.04 hoặc Debian 12. Việc build trên distro rolling-release rồi chép binary sang distro cũ có thể gây lỗi phiên bản `GLIBC`.

### Muốn xem toàn bộ lệnh Makefile

```bash
make help
```

## Giấy phép

Mã nguồn wrapper này được phát hành theo [MIT License](LICENSE).

Giấy phép không áp dụng cho Zalo, nội dung của Zalo hoặc các nhãn hiệu thuộc
VNG Corporation và các chủ sở hữu tương ứng. Project này không liên kết hay
được xác nhận chính thức bởi Zalo/VNG.
