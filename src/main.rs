use std::{
    io::{self, Write},
    process, thread, time,
};

use rand::Rng;

fn acak() -> i32 {
    rand::thread_rng().gen_range(0..=100)
}

fn shutdown() {
    process::Command::new("systemctl")
        .arg("poweroff")
        .output()
        .expect("shutdown error");
}

fn clear_terminal(){
    process::Command::new("clear").status().expect("ini error");
}

fn main() {
    let mut input_user = String::new();
    println!("angka 0-100");
    print!("masukan angka target: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_user).unwrap();

    let input_user: i32 = input_user.trim().parse().expect("harus angka");

    println!("target pencarian: {}", input_user);

    loop {
        let target = acak();
        println!("{}", target);
        thread::sleep(time::Duration::from_secs(2));
        clear_terminal();

        if target == input_user {
            println!("target terselesaikan.....");
            println!("komputer dimatikan....");
            shutdown();
        }
    }
}
