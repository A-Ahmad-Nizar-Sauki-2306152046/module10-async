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

