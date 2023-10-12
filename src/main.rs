use std::{
    io::{self, Write},
    process, thread, time,
};

use rand::Rng;

fn acak() -> i32 {
    rand::thread_rng().gen_range(0..250)
}

fn shutdown() {
    process::Command::new("systemctl")
        .arg("poweroff")
        .output()
        .expect("shutdown error");
}

fn main() {
    let mut input_user = String::new();
    print!("masukan angka target: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_user).unwrap();

    let input_user: i32 = input_user.trim().parse().expect("harus angka");

    println!("target pencarian: {}", input_user);

    loop {
        let target = acak();
        println!("{}", target);
        thread::sleep(time::Duration::from_secs(2));

        if target == input_user {
            println!("komputer dimatikan....");
            shutdown();
        }
    }
}
