# Tutorial 2 - Broadcast

## Experiment 2.1: Original Code, and How it Run

### Deskripsi
Pada eksperimen ini, saya menjalankan kode broadcast chat original dari 
Google Comprehensive Rust. Aplikasi ini menggunakan WebSocket untuk 
komunikasi real-time antara satu server dan beberapa client.

### Cara Menjalankan

#### 1. Jalankan Server
```bash
cargo run --bin server
```

#### 2. Jalankan Client (buka terminal baru untuk setiap client)
```bash
cargo run --bin client
```

### Yang Terjadi
- Saat server dijalankan, server mulai mendengarkan koneksi pada port 2000
- Setiap client yang terhubung akan menerima pesan sambutan: 
  **"Welcome to chat! Type a message"**
- Ketika satu client mengirim pesan, server akan **menyebarkan (broadcast)** 
  pesan tersebut ke **semua client yang sedang terhubung**
- Di sisi server, setiap pesan yang masuk akan ditampilkan beserta 
  alamat IP dan port pengirimnya

### Hasil Percobaan
Pada percobaan ini, saya menjalankan 1 server dan 2 client sekaligus.
- Client 1 mengirim pesan → diterima oleh Client 2, begitu pula sebaliknya
- Server mencatat setiap pesan masuk dari client beserta alamat asalnya

> <img width="2550" height="1176" alt="Image" src="https://github.com/user-attachments/assets/57d4a1c5-51ac-4729-9ed3-f66610b5ba1c" />

## Experiment 2.2: Modifying Port

### Deskripsi
Pada eksperimen ini, saya mengubah port WebSocket dari **2000** menjadi **8080**.
Karena WebSocket adalah protokol berbasis koneksi dua arah, perubahan port 
harus dilakukan di dua tempat sekaligus.

### Perubahan yang Dilakukan

#### `src/bin/server.rs`
```rust
// Sebelum
let listener = TcpListener::bind("127.0.0.1:2000").await?;
println!("listening on port 2000");

// Sesudah
let listener = TcpListener::bind("127.0.0.1:8080").await?;
println!("listening on port 8080");
```

#### `src/bin/client.rs`
```rust
// Sebelum
ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))

// Sesudah
ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
```

### Penjelasan
Koneksi WebSocket melibatkan dua sisi: **server** (yang mendengarkan) 
dan **client** (yang menghubungkan diri). Keduanya harus menggunakan 
port yang sama agar koneksi berhasil terbentuk. Jika hanya salah satu 
yang diubah, client tidak akan bisa terhubung ke server.

### Hasil Percobaan
Seperti terlihat pada screenshot:
- Server berjalan dan mendengarkan di **port 8080**
- Dua client berhasil terhubung dari `127.0.0.1:52901` dan `127.0.0.1:52902`
- Client 1 mengirim pesan **"client 1 hadir"** → diterima Client 2
- Client 2 mengirim pesan **"client 2 juga"** → diterima Client 1
- Semua pesan berhasil dibroadcast dengan benar melalui port baru

> <img width="2026" height="642" alt="Image" src="https://github.com/user-attachments/assets/7c959067-e8be-4410-93c3-6ad5650a8ac5" />

## Experiment 2.3: Small Changes, Add IP and Port

### Deskripsi
Pada eksperimen ini, saya memodifikasi server agar setiap pesan yang 
dibroadcast menyertakan informasi **IP dan Port** dari pengirimnya.
Dengan begitu, setiap client bisa mengetahui dari mana asal pesan tersebut.

### Perubahan yang Dilakukan

#### `src/bin/server.rs`
Pada fungsi `handle_connection`, pesan yang diterima dari client 
sebelum dibroadcast dibungkus dengan format yang menyertakan alamat pengirim:

```rust
// Sebelum
bcast_tx.send(text.into())?;

// Sesudah
let message_with_sender = format!("[{}]: {}", addr, text);
bcast_tx.send(message_with_sender)?;
```

### Penjelasan
Variabel `addr` bertipe `SocketAddr` sudah tersedia sebagai parameter 
di fungsi `handle_connection`. Dengan menggunakan `format!()`, kita 
sisipkan informasi IP dan Port pengirim ke dalam setiap pesan sebelum 
disebarkan ke semua client. Hal ini berguna agar setiap client tahu 
siapa yang mengirim pesan, meskipun belum ada sistem nama pengguna.

### Hasil Percobaan
Seperti terlihat pada screenshot:
- Server menerima koneksi dari `127.0.0.1:52702` dan `127.0.0.1:52703`
- Setiap pesan yang diterima client kini tampil dalam format `[IP:Port]: pesan`
  - `[127.0.0.1:52702]: hello saya`
  - `[127.0.0.1:52703]: hai kamu`
  - `[127.0.0.1:52702]: test`
  - `[127.0.0.1:52702]: client 1`
  - `[127.0.0.1:52703]: client 2`
- Semua client bisa melihat dari port mana setiap pesan berasal

> <img width="2028" height="628" alt="Image" src="https://github.com/user-attachments/assets/cb2dfdae-b4f6-4f3b-bb57-2790283a3203" />