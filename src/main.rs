fn main() {
    println!("Hello, world!");
}

/*
 * saat menjalankan aplikasi, aplikasi akan dijalankan dalam "process", process akan diatur oleh OS.
 * didalam process, kita bisa membuat "thread" untuk menjalankan kode secara parallel.
 * di rust, kita bisa menggunakan module std::thread untuk membuat thread.
 */

#[cfg(test)]
mod tests {

    use std::{thread, time::Duration};

    /*
     * untuk membuat thread baru yang berjalan secara parallel dan async, kita bisa menggunakan
     * std::thread::spawn(closure)
     */
    #[test]
    fn thread_test() {
        thread::spawn(|| {
            for i in 1..=5 {
                println!("{}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("application finished!"); // this will be run first
        thread::sleep(Duration::from_secs(7));
    }
}
