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
        sync::mpsc::{self, channel},
        thread::{self, Builder, JoinHandle, sleep},
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
        let cur_thread = thread::current();
        let thread_name = cur_thread.name().unwrap();
        println!("using thread name: {}", thread_name);
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

    // current thread
    // semua program di rust, defaultnya jalan di dalam thread, meskipun kita tidak mendefinisikan
    // kalau ini dijalankan di thread.
    // termasuk saat menjalankan unit test. thread akan dibuat secara otomatis lah intinya
    // untuk mendapatkan thread yang sedang berjalan sekarang, bisa menggunakan thread::current
    //
    #[test]
    fn current_thread_test() {
        let t: thread::Thread = thread::current();
        let t_name = t.name().unwrap();
        println!("{}", t_name);
    }

    // Thread Factory
    /*
     * Thread factory adalah object untuk membuat thread.
     * Saat membuat thread, sebenernya kita menggunakan default Thread Factory Bawaan Rust.
     * secara default, rust sudah membuat default Thread factory, dan ketika menggunakan thread::spawn(),
     * maka kita akan memanggil default thread factory tersebut.
     * Kita juga bisa sih sebenernya bikin thread factory secara manual, dengan cara
     * `threead::Builder::new()`
     * Hal ini mungkin dibutuhkan ketika kita ingin mengatur konfigurasi thread factory, atau nanti
     * menggunakan library/framework yang membutuhkan thread factory.
     */

    #[test]
    fn thread_factory_test() {
        let builder: Builder = Builder::new().name("custom builder".to_string());
        let handler: JoinHandle<i32> = builder
            .spawn(|| calculate())
            .expect("something went wrong...");

        let _ = handler.join().unwrap();

        println!("asoy")
    }

    /*
     * Channel
     * merupakan struktur data  mirip queue, dimana thread ini bisa mengirim data ke channel, dan
     * bisa menerima data dari channel
     *
     * antar thread itu tidak ada direct communication, harus melalui channel
     * di dalam channel, terdapat 2 pihak, sender dan receiver
     * thread dalam satu waktu bisa bertindak sebagai sender dan receiver
     * channel ini, ada rust di module `mpsc`
     */
    #[test]
    fn channel_test() {
        let (sender, receiver) = channel::<String>();

        let handle1 = thread::spawn(move || {
            sleep(Duration::from_secs(2));
            let _ = sender.send("Hello from handle1".to_string());
        });

        let handle2 = thread::spawn(move || {
            let result = receiver.recv().unwrap();
            println!("result: {}", result);
        });

        let _ = handle1.join();
        let _ = handle2.join();

        println!("done...");
    }

    #[test]
    fn channel_multiple_data_test() {
        let (sender, receiver) = channel::<String>();

        let handle1 = thread::spawn(move || {
            sleep(Duration::from_secs(2));
            for i in 1..=5 {
                let text: String = format!("Hello from handle1 iter {}", i);
                let _ = sender.send(text);
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("result: {}", value);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();

        println!("done...");
    }

    #[test]
    fn multi_sender_test() {
        /*
         * dari awal kita tau bahwa module untuk channel bername Multi Producer Single Consumer.
         * artinya kita tau bahwa harusnya kita bisa membuat multi sender / multi Producer
         * kita bisa melakukan nya dengan cara clone data Sender, secara otomatis, sender hasil
         * clone akan mengirim ke receiver yang sama
         */
        let (sender, receiver) = mpsc::channel::<String>();
        let sender_two = sender.clone(); // ini bisa kita clone sendernya

        let handle1 = thread::spawn(move || {
            sleep(Duration::from_secs(1));
            for i in 1..=20 {
                let text: String = format!("Hello from handle1 iter {}", i);
                let _ = sender.send(text);

                sleep(Duration::from_millis(150));
            }
        });

        let handle2 = thread::spawn(move || {
            sleep(Duration::from_secs(2));
            for i in 1..=20 {
                let text: String = format!("Hello from handle 2 iter {}", i);
                let _ = sender_two.send(text);

                sleep(Duration::from_millis(150));
            }
        });

        let receive = thread::spawn(move || {
            for v in receiver.iter() {
                println!("got: {}", v);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
        let _ = receive.join();
    }
}
