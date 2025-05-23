![alt text](image.png)
Penjelasan: Penambahan baris println!("Raja's Computer: hey hey hey!"); di dalam fungsi main() akan mencetak teks itu secara langsung setelah memanggil spawner.spawn(), dan sebelum eksekusi
asynchronous dimulai.

![alt text](image-1.png)
Penjelasan: Output program berbeda-beda karena ketiga task async (howdy, howdy2, howdy3) dijalankan secara bersamaan (concurrently).
Rust executor tidak menjamin urutan eksekusi antar task, sehingga hasil done, done2, dan done3 bisa muncul dalam urutan yang berubah-ubah setiap kali dijalankan.