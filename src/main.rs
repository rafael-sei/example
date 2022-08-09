use std::io;

fn main() {
    let mut temperature_type = String::new();
    println!("実行したい内容を入力して下さい(0:華氏 -> 摂氏, 1:摂氏 -> 華氏)");
    io::stdin().read_line(&mut temperature_type).unwrap();
    let temperature_type: u8 = temperature_type.trim().parse().expect("数値を入力して下さい");

    if temperature_type == 0 {
        println!("華氏を入力して下さい");
    } else if temperature_type == 1 {
        println!("摂氏を入力して下さい");
    } else {
        println!("入力が不正です");
        panic!();
    }

    temperature();
}

fn temperature() {
    print!("華氏を入力して下さい: ");
    // io::stdout().flush().unwrap(); // 横に並べたければ
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();

    let inp: i32 = inp.trim().parse().unwrap();

    let degrees = Degrees {
        c: inp,
    };
    println!("{}", degrees.f_to_c());
}

struct Degrees {
    c: i32,
}

impl Degrees {
    fn f_to_c(&self) -> i32 {
         (self.c - 32) * 5 / 9
    }

    fn _c_to_f(&self) -> i32 {
        (self.c * 9 / 5) + 32
    }
}
