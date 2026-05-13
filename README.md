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
