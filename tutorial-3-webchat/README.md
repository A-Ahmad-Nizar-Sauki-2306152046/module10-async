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