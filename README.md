# Concurrency
Nama: Hadyan Fachri\
Kelas: Advprog A\
NPM: 2306245030

## Commit 1 Reflection Notes
*Milestone 1: Single threaded web server*
Pada milestone pertama, saya mengembangkan web server dengan menambahkan fungsi `handle_connection()` agar bisa membaca dan mencetak permintaan HTTP dari klien. Dengan `BufReader`, server dapat membaca setiap baris request, menyimpannya dalam `Vec<String>`, lalu menampilkannya di terminal. Fungsi ini memastikan hanya bagian header request yang diambil dengan `.take_while(|line| !line.is_empty())`, sehingga bisa melihat detail seperti metode HTTP, path, dan user-agent. Saat dijalankan dengan `cargo run` dan diakses melalui browser, server akan mencetak daftar header request yang masuk, menandakan bahwa koneksi berhasil diterima dan diproses.

## Commit 2 Reflection Notes
*Milestone 2: Returning HTML*
Pada milestone kedua, saya meningkatkan fungsionalitas web server dengan memungkinkan server mengirimkan file HTML sebagai respons terhadap permintaan HTTP. Sebelumnya, server hanya mencetak request yang diterima ke terminal, tetapi sekarang perlu mengimplementasikan fungsi `handle_connection()` yang membaca file HTML (`hello.html`), lalu mengirimkannya ke klien dengan format HTTP yang sesuai. Dengan menggunakan `fs::read_to_string()` yang dapat membaca konten file, lalu menyusunnya menjadi respons lengkap dengan status `HTTP/1.1 200 OK` dan header `Content-Length`. Hasilnya, saat mengakses **http://127.0.0.1:7878** melalui browser akan melihat halaman HTML yang telah dibuat. Perubahan ini memberikan dasar bagi server untuk menangani berbagai jenis permintaan dan meresponsnya dengan konten yang lebih kompleks di tahap berikutnya.

![Commit 2 Screen capture](commit2.png)

## Commit 3 Reflection Notes
*Milestone 3: Validating request and selectively responding* 
Pada milestone ketiga, saya meningkatkan server dengan menambahkan **validasi request** dan **penanganan respons yang berbeda** berdasarkan path yang diminta. Dengan menggunakan `BufReader`, program dapat membaca baris pertama request (`request_line`) untuk mengetahui **path yang diminta oleh klien**. Jika permintaan adalah `"GET / HTTP/1.1"`, server akan merespons dengan file **hello.html** dan status `"HTTP/1.1 200 OK"`. Jika path yang diminta berbeda, server akan mengembalikan **404 Page.html** dengan status `"HTTP/1.1 404 NOT FOUND"`.

Dengan pendekatan ini, server kini dapat membedakan antara request yang valid dan yang tidak ditemukan, seperti server web nyata yang menampilkan **halaman utama** atau **halaman error 404** jika pengguna mencoba mengakses path yang tidak tersedia. Hal ini menjadi dasar bagi penanganan request yang lebih kompleks di milestone berikutnya.

![Commit 3 Screen Capture](commit3.png)

## Commit 4 Reflection Notes
*Milestone 4: Simulation slow response* 
Pada milestone keempat, saya mensimulasikan **respon server yang lambat** untuk memahami bagaimana server single-threaded menangani permintaan secara berurutan. Dengan menambahkan **path `/sleep`**, perlu menggunakan `thread::sleep(Duration::from_secs(10))` untuk membuat server **menunda respons selama 10 detik** sebelum mengirimkan halaman HTML. Ketika satu permintaan `/sleep` sedang diproses, permintaan lain (misalnya akses ke `/`) akan tertunda hingga proses sebelumnya selesai.

Ini menunjukkan **keterbatasan server single-threaded**, di mana satu permintaan yang lama dapat memblokir permintaan lainnya. Jika dua tab browser mencoba mengakses server secara bersamaan—satu menuju `/sleep` dan satu ke `/`—maka permintaan ke `/` juga akan tertunda hingga proses `/sleep` selesai. Terbukti bahwa pendekatan **single-threaded tidak efisien untuk menangani banyak request secara bersamaan**, sehingga di milestone berikutnya akan beralih ke **multi-threading** untuk meningkatkan performa server.

## Commit 5 Reflection Notes
*Milestone 5: Multithreaded Server*
Pada milestone terakhir, saya meningkatkan performa web server dengan menerapkan **ThreadPool**, sehingga server dapat menangani beberapa koneksi secara bersamaan tanpa harus menunggu setiap permintaan selesai diproses. Dengan menggunakan **multiple worker threads**, program dapat menghindari bottleneck yang terjadi pada implementasi single-threaded di milestone sebelumnya.  

Implementasi **ThreadPool** menggunakan **mpsc (multi-producer, single-consumer) channel** untuk mengirim tugas dari main thread ke worker threads. **Arc<Mutex<T>>** digunakan untuk memastikan **sinkronisasi antar-thread**, sehingga banyak worker dapat mengakses job queue dengan aman tanpa race condition. Setiap worker berjalan dalam **loop** dan mengambil tugas dari queue, lalu mengeksekusinya.  

Salah satu tantangan dalam milestone ini adalah **mengelola lifecycle dari worker threads**. Kode yang dibuat belum memiliki mekanisme shutdown yang baik, sehingga ketika server berhenti, thread pekerja tetap berjalan tanpa cara yang jelas untuk dihentikan. Selain itu, penggunaan `unwrap()` tanpa error handling dapat menyebabkan panic jika ada kesalahan saat menerima tugas dari channel.  

Dari milestone ini, dapat diambil bagaimana **memanfaatkan concurrency untuk meningkatkan efisiensi server**, serta pentingnya **sinkronisasi antar-thread** agar aplikasi tetap stabil. Implementasi yang lebih lengkap sebaiknya mencakup mekanisme shutdown yang memungkinkan thread pekerja keluar dengan aman saat server berhenti.