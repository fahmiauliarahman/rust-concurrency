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

    use std::{
        thread::{self, JoinHandle},
        time::{Duration, Instant},
    };

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

    /*
     * saat kita menjalankan thread menggunakan spawn function, maka dia akan mengembalikan
     * JoinHandle<T>.
     * JoinHandler bisa digunakan untuk join thread dengan memanggil method join()
     * menurut gw ini mirip sama Promise nya javascript tapi gatau deh...
     */

    #[test]
    fn join_thread_test() {
        let handler: JoinHandle<i32> = thread::spawn(|| {
            let mut counter = 0;

            for i in 1..=5 {
                println!("iter: {}", i);
                counter += 1
            }

            counter
        });

        let result = handler.join(); // blocking -> menunggu thread process sampai selesai

        match result {
            Ok(result) => {
                println!("counter: {}", result);
            }
            Err(e) => println!("something went wrong: {:?}", e),
        };

        println!("Application finished!");
    }

    fn calculate() -> i32 {
        let mut counter: i32 = 0;

        for i in 1..=5 {
            println!("iter: {}", i);
            counter += 1;
            thread::sleep(Duration::from_secs(1));
        }

        counter
    }

    #[test]
    fn sequential_test() {
        let start = Instant::now();
        let _ = calculate();
        let _ = calculate();

        let duration = start.elapsed();

        println!("took {}s", duration.as_secs_f64())
    }

    #[test]
    fn using_thread_test() {
        let start = Instant::now();

        let result = thread::spawn(calculate);
        let result2 = thread::spawn(calculate);

        let result = result.join();
        let result2 = result2.join();

        match result {
            Ok(_) => println!("result 1 finished with no error"),
            Err(e) => println!("finished with error: {:?}", e),
        }

        match result2 {
            Ok(_) => println!("result 2 finished with no error"),
            Err(e) => println!("finished with error: {:?}", e),
        }

        let duration = start.elapsed();

        println!("took {}s", duration.as_secs_f64())
    }

    #[test]
    fn move_ownership_test() {
        let name: String = "Fahmi".to_string();

        // move -> ownership diluar scope yang lagi dipake akan dipindah.
        let handle = move || {
            thread::sleep(Duration::from_secs(1));
            println!("Hello {}", name)
        };

        //handle();

        // tanpa move sebelum closure, ini akan error, karena rust ga memperbolehkan variable diluar
        // scope dari thread::spawn. solusinya adalah dengan menambahkan move sebelum closure.
        let exec = thread::spawn(handle);

        exec.join().unwrap();

        // kalau ada move nya, kita jadi ga bisa mengakses variable name nya karena ownershipnya
        // udah dipindah.
        // println!("name: {}", name);

        println!("Finished!")
    }
}
