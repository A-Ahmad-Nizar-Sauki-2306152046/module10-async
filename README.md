# Tutorial 1 - Timer
### Experiment 1.2 - Understanding How It Works

<img width="2062" height="394" alt="Image" src="https://github.com/user-attachments/assets/f42fe412-e35a-49e6-840d-7c7f2205bb92" />

Output:
```
Nizar's Komputer: hey hey!
Nizar's Komputer: howdy!
Nizar's Komputer: done!
```

**Penjelasan:**
`hey hey!` muncul paling pertama karena spawner.spawn() tidak langsung
menjalankan async block, melainkan hanya mendaftarkan task ke antrian executor.
Kode setelah spawner.spawn() tetap berjalan secara synchronous. Executor baru
benar-benar mengeksekusi task ketika executor.run() dipanggil di akhir main().


### Experiment 1.3 - Multiple Spawn and Removing Drop

**Multiple Spawn:**

> <img width="2080" height="592" alt="Image" src="https://github.com/user-attachments/assets/0f0a5a89-e689-48d8-98f8-ae67e2ed01a6" />

Ketiga task dijalankan secara concurrent oleh executor. Output howdy1, howdy2,
howdy3 muncul, lalu setelah 2 detik muncul done1, done2, done3.

**Menghapus drop(spawner):**

> <img width="1864" height="706" alt="Image" src="https://github.com/user-attachments/assets/c2dbdafb-e74f-4e46-9e6e-282e9ed19fea" />

Ketika drop(spawner) dihapus, program tidak pernah berhenti (hang).
Ini karena executor.run() terus menunggu task baru dari spawner.
drop(spawner) berfungsi sebagai sinyal ke executor bahwa tidak akan
ada task baru lagi, sehingga executor tahu kapan harus berhenti.

**Kesimpulan:**
- Spawner  : bertugas mendaftarkan/mengirim task ke antrian executor
- Executor : bertugas menjalankan semua task yang ada di antrian
- drop()   : memberitahu executor bahwa spawner sudah selesai,
             sehingga executor bisa berhenti setelah semua task selesai

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

# Tutorial 3 -  WebChat

## Experiment 3.1: Original Code

### Deskripsi
Pada eksperimen ini, saya menjalankan kode original YewChat — aplikasi 
webchat berbasis browser yang dibangun menggunakan Rust dan framework Yew. 
Frontend dikompilasi ke WebAssembly (WASM) dan berkomunikasi dengan 
WebSocket server yang ditulis dalam TypeScript.

### Cara Menjalankan

#### 1. Jalankan WebSocket Server
```bash
cd SimpleWebsocketServer
npm i
npm start
# Server berjalan di port 8080
```

#### 2. Jalankan Frontend YewChat
```bash
cd YewChat
npm i
npm start
# Frontend berjalan di localhost:8000
```

### Catatan Setup
Karena project ini menggunakan wasm-bindgen versi lama (0.2.45), 
diperlukan Rust versi 1.77.0 agar kompatibel. Gunakan:
```bash
rustup toolchain install 1.77.0
rustup default 1.77.0
```

### Hasil Percobaan
- Halaman login muncul dengan input Username dan tombol GO CHATTING!
- Setelah masuk, tampil halaman chat dengan daftar Users di sidebar kiri
- User yang terhubung muncul di panel Users beserta status "Hi there!"
- Pesan dapat dikirim melalui input Message di bagian bawah

> <img width="2560" height="1420" alt="Image" src="https://github.com/user-attachments/assets/dba97d88-51f2-4edd-ab3d-66af91162395" />

> <img width="2560" height="1424" alt="Image" src="https://github.com/user-attachments/assets/54be126b-9c16-4738-8ffd-affa7f1bf037" />

## Experiment 3.2: Be Creative!

### Deskripsi
Pada eksperimen ini, saya memodifikasi tampilan YewChat dengan mengubah 
warna tema dan menambahkan emoji agar lebih menarik dan personal.

### Perubahan yang Dilakukan

#### 1. Halaman Login (`src/components/login.rs`)
- Mengubah warna background dari `bg-gray-800` menjadi `bg-indigo-900` (biru gelap)
- Menambahkan judul **"💬 YewChat"** di atas form
- Menambahkan teks sambutan: *"Selamat datang! Masukkan username untuk mulai chat 🚀"*
- Mengubah warna tombol dari `bg-violet-600` menjadi `bg-emerald-500` (hijau)
- Menambahkan emoji 🚀 pada tombol "GO CHATTING!"
- Menambahkan emoji ✏️ pada placeholder input username

#### 2. Halaman Chat (`src/components/chat.rs`)
- Mengubah warna sidebar dari `bg-gray-100` menjadi `bg-indigo-900` (biru gelap)
- Mengubah warna card user dari `bg-white` menjadi `bg-indigo-700`
- Mengubah status user dari "Hi there!" menjadi "🟢 Online"
- Menambahkan emoji 👥 pada header "Users"
- Menambahkan emoji 💬 pada header "YewChat!" dan nama pengirim pesan
- Mengubah warna tombol kirim dari `bg-blue-600` menjadi `bg-emerald-500`
- Mengubah placeholder input dari "Message" menjadi "✏️ Ketik pesan..."

### Hasil Percobaan
- Halaman login kini memiliki tema biru gelap dengan aksen hijau
- Sidebar chat berwarna biru gelap senada dengan halaman login
- Emoji ditambahkan di berbagai elemen untuk tampilan yang lebih hidup

> <img width="2560" height="1424" alt="Image" src="https://github.com/user-attachments/assets/71f2626b-02a6-4d9d-9c78-40f6ee9ddfce" />
> <img width="2560" height="1430" alt="Image" src="https://github.com/user-attachments/assets/1e75c795-6f2b-43c3-a327-2952f3fcd359" />

# Bonus: Rust Websocket Server for YewChat!

## Deskripsi
Pada bagian bonus ini, saya mengganti WebSocket server JavaScript (TypeScript) 
dari Tutorial 3 dengan server Rust yang dimodifikasi dari Tutorial 2. 
Tantangannya adalah menyesuaikan format komunikasi karena YewChat menggunakan 
JSON, sedangkan server Tutorial 2 awalnya hanya mengirim plain text.

## Perbedaan Format Pesan

### Tutorial 2 (plain text)
misal:
```
hello
hai kamu
```

### Tutorial 3 / YewChat (JSON)
```json
// Client → Server (register)
{"messageType":"register","data":"Person 1"}

// Client → Server (pesan)
{"messageType":"message","data":"hello"}

// Server → Client (daftar users)
{"messageType":"users","dataArray":["Person 1","Person 2"]}

// Server → Client (pesan broadcast)
{"messageType":"message","data":"{\"from\":\"Person 1\",\"message\":\"hello\"}"}
```

Meskipun formatnya berbeda, keduanya tetap dikirim sebagai **satu text message** 
melalui WebSocket. JSON hanya di-serialize menjadi string teks, sehingga 
protokol WebSocket-nya tetap sama.

## Perubahan pada Server Rust (Tutorial 2)

Modifikasi dilakukan pada `src/bin/server.rs`:

1. Menambahkan dependency `serde` dan `serde_json` untuk serialisasi JSON
2. Menambahkan struct `IncomingMessage`, `OutgoingUsers`, `OutgoingMessage`, 
   dan `MessageData` untuk parsing dan formatting pesan JSON
3. Menambahkan `HashMap<SocketAddr, String>` untuk menyimpan daftar user 
   yang terhubung beserta username-nya
4. Menangani dua jenis pesan:
   - `register` → simpan username, broadcast daftar user terbaru
   - `message` → broadcast pesan beserta info pengirim
5. Menghapus user dari daftar saat koneksi terputus dan broadcast ulang

## Catatan Versi Rust

Proyek ini menggunakan **dua versi Rust yang berbeda**:

| Komponen | Versi Rust | Alasan |
|----------|-----------|--------|
| Server Rust (Tutorial 2) | stable (1.94.1) | Tidak ada constraint versi |
| Frontend YewChat | 1.77.0 | `wasm-bindgen = "0.2.45"` tidak kompatibel dengan Rust > 1.77.0 |

Cara berpindah versi:
```bash
# Untuk menjalankan server
rustup default stable
cargo run --bin server

# Untuk menjalankan YewChat
rustup default 1.77.0
npm start
```

## Hasil Percobaan
Seperti terlihat pada screenshot dan output terminal:
- Server Rust berhasil menerima koneksi dari dua client
- Pesan JSON berhasil di-parse dan di-broadcast dengan benar
- YewChat menampilkan daftar user dan pesan secara real-time

## Pendapat: JavaScript vs Rust

**JavaScript (TypeScript)** lebih mudah untuk disetup dan dikembangkan 
dengan cepat. Cocok untuk prototyping karena ekosistem npm yang lengkap 
dan sintaks yang lebih familiar.

**Rust** lebih unggul dalam hal performa dan keamanan memori. Server Rust 
tidak membutuhkan garbage collector dan lebih efisien dalam menangani 
banyak koneksi bersamaan. Namun setup-nya lebih kompleks, terutama 
karena masalah kompatibilitas versi seperti yang dialami di tutorial ini.

Untuk production dengan beban tinggi, saya lebih memilih **Rust**. 
Namun untuk pengembangan cepat, **JavaScript/TypeScript** tetap lebih praktis.

> <img width="2560" height="1424" alt="Image" src="https://github.com/user-attachments/assets/17eec17d-4c65-48aa-99a5-b137a115f8f6" />
> <img width="2560" height="1426" alt="Image" src="https://github.com/user-attachments/assets/43b0755b-36a8-4dbd-90a1-4ecffedfca13" />