# Concurrency
Nama: Hadyan Fachri\
Kelas: Advprog A\
NPM: 2306245030\

## Commit 1 Reflection Notes
*Milestone 1: Single threaded web server*
Pada milestone pertama, saya mengembangkan web server dengan menambahkan fungsi `handle_connection()` agar bisa membaca dan mencetak permintaan HTTP dari klien. Dengan `BufReader`, server dapat membaca setiap baris request, menyimpannya dalam `Vec<String>`, lalu menampilkannya di terminal. Fungsi ini memastikan hanya bagian header request yang diambil dengan `.take_while(|line| !line.is_empty())`, sehingga bisa melihat detail seperti metode HTTP, path, dan user-agent. Saat dijalankan dengan `cargo run` dan diakses melalui browser, server akan mencetak daftar header request yang masuk, menandakan bahwa koneksi berhasil diterima dan diproses.

## Commit 2 Reflection
*Milestone 2: Returning HTML*
Pada milestone ini, saya meningkatkan fungsionalitas web server dengan memungkinkan server mengirimkan file HTML sebagai respons terhadap permintaan HTTP. Sebelumnya, server hanya mencetak request yang diterima ke terminal, tetapi sekarang perlu mengimplementasikan fungsi `handle_connection()` yang membaca file HTML (`hello.html`), lalu mengirimkannya ke klien dengan format HTTP yang sesuai. Dengan menggunakan `fs::read_to_string()` yang dapat membaca konten file, lalu menyusunnya menjadi respons lengkap dengan status `HTTP/1.1 200 OK` dan header `Content-Length`. Hasilnya, saat mengakses **http://127.0.0.1:7878** melalui browser akan melihat halaman HTML yang telah kita buat. Perubahan ini memberikan dasar bagi server untuk menangani berbagai jenis permintaan dan meresponsnya dengan konten yang lebih kompleks di tahap berikutnya.