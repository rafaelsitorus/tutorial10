![alt text](image.png)
Penjelasan: Penambahan baris println!("Raja's Computer: hey hey hey!"); di dalam fungsi main() akan mencetak teks itu secara langsung setelah memanggil spawner.spawn(), dan sebelum eksekusi
asynchronous dimulai.

![alt text](image-1.png)
Penjelasan: Output program berbeda-beda karena ketiga task async (howdy, howdy2, howdy3) dijalankan secara bersamaan (concurrently).
Rust executor tidak menjamin urutan eksekusi antar task, sehingga hasil done, done2, dan done3 bisa muncul dalam urutan yang berubah-ubah setiap kali dijalankan.

![alt text](image-2.png)
Penjelasa: Program ini adalah contoh obrolan WebSocket dasar yang dibangun dengan Rust, menggunakan tokio untuk operasi asinkron. Cukup jalankan cargo run --bin server di satu terminal, lalu cargo run --bin client di beberapa terminal lain. Setiap kali mengetik pesan di salah satu klien dan menekan Enter, pesan itu akan dikirim ke server, yang kemudian menyiarkannya kembali ke semua klien yang terhubung, memungkinkan semua orang melihat pesan satu sama lain secara real-time.