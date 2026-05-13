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