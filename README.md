# Zalo for Linux (Tauri)

Ứng dụng desktop Linux tối giản cho `https://chat.zalo.me/`, viết bằng Tauri v2 và WebKitGTK. Project không đóng gói hay sửa mã nguồn Zalo; nó chỉ tạo cửa sổ native, system tray và tích hợp notification cho Zalo Web.

## Tính năng

- Thanh title custom nhỏ gọn với nút thu nhỏ, phóng to và đóng.
- Kéo cửa sổ bằng vùng title; double-click để phóng to hoặc khôi phục.
- Đóng cửa sổ sẽ ẩn xuống system tray thay vì thoát.
- Menu tray gồm **Mở Zalo**, **Ẩn Zalo** và **Thoát hoàn toàn**.
- Chỉ cho phép một instance. Mở lần hai sẽ hiện và focus cửa sổ đang chạy.
- Chuyển Web Notification của Zalo thành notification native KDE/Linux.
- Tự khởi động sau khi đăng nhập desktop.
- Nội dung từ xa không được cấp Tauri command/capability.

## Yêu cầu hệ thống

Project được phát triển và kiểm tra trên Arch Linux, KDE Plasma, Wayland.

Cài thư viện build cần thiết:

```bash
sudo pacman -S --needed base-devel gtk3 webkit2gtk-4.1 libayatana-appindicator
```

Cần Rust stable có `cargo` và `rustc`. Có thể dùng gói Arch:

```bash
sudo pacman -S --needed rust
```

Hoặc dùng `rustup` nếu hệ thống đã quản lý Rust theo cách đó. Kiểm tra toolchain:

```bash
cargo --version
rustc --version
```

## Build

Đi vào thư mục project:

```bash
cd ~/Desktop/zalo-tauri
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

Thông báo tin nhắn dùng notification native của desktop. Trong Zalo Web vẫn cần bật thông báo tại **Cài đặt → Thông báo** và KDE không được đặt ứng dụng ở chế độ tắt thông báo.

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
    └── src/main.rs                Tray, title bar, notification, single-instance
```

## Xử lý lỗi

### App không xuất hiện nhưng tiến trình vẫn chạy

Chọn **Mở Zalo** từ tray hoặc chạy:

```bash
make start
```

### Không có tray icon

KDE Plasma đã hỗ trợ StatusNotifierItem. Kiểm tra `libayatana-appindicator` đã được cài và khởi động lại app:

```bash
sudo pacman -S --needed libayatana-appindicator
make restart
```

### Không nhận được thông báo

1. Bật thông báo trong cài đặt Zalo Web.
2. Kiểm tra **System Settings → Notifications** của KDE.
3. Giữ ứng dụng chạy dưới tray.
4. Xem log bằng `make logs`.

### Build lỗi do thiếu WebKitGTK

```bash
sudo pacman -S --needed webkit2gtk-4.1 gtk3
make build
```

### Muốn xem toàn bộ lệnh Makefile

```bash
make help
```
