# Concurrency
Nama: Hadyan Fachri\
Kelas: Advprog A\
NPM: 2306245030\

## Commit 1 Reflection Notes
*Milestone 1: Single threaded web server*
Pada milestone pertama, kita mengembangkan web server dengan menambahkan fungsi `handle_connection()` agar bisa membaca dan mencetak permintaan HTTP dari klien. Dengan `BufReader`, server dapat membaca setiap baris request, menyimpannya dalam `Vec<String>`, lalu menampilkannya di terminal. Fungsi ini memastikan hanya bagian header request yang diambil dengan `.take_while(|line| !line.is_empty())`, sehingga kita bisa melihat detail seperti metode HTTP, path, dan user-agent. Saat dijalankan dengan `cargo run` dan diakses melalui browser, server akan mencetak daftar header request yang masuk, menandakan bahwa koneksi berhasil diterima dan diproses.