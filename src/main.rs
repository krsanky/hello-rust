use std::thread;
use std::time;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);
        let d = time::Duration::from_millis(1000);
        thread::sleep(d);
        println!("{} is done eating.", self.name);
    }
}

fn f1() {
    let x = 5 + /* 90 + */ 5;

    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_int = 5i32;

    println!("x = {}", x);
    println!("logical:{} flt:{} int:{}", logical, a_float, an_int);

    let _def_flt = 3.0; // f64
    let _def_int = 7; // i32

    let mut inferred_type = 12;
    println!("inf_type:{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("inf_type:{}", inferred_type);

    let inferred_type = true;
    println!("inf_type:{}", inferred_type);

    println!("one million :{}", 1_000_000u32);
}

fn cl1() {
    let color = "green";
    let print = || println!("`color`:{}", color);

    print();
    print();
}

fn main() {
    f1();
    cl1();

    println!("-----------");
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                p.eat();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
