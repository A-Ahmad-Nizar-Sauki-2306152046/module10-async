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