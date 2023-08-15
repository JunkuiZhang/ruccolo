use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

static DATA: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();

fn main() {
    let mut d1 = HashMap::new();
    d1.insert(0, 0);
    d1.insert(1, 1);
    d1.insert(2, 2);
    d1.insert(3, 3);
    // DATA.set(Mutex::new(d1)).unwrap();
    DATA.set(Mutex::new(d1)).unwrap();

    {
        let temp = DATA.get().unwrap().lock().unwrap();
        for x in 0..4usize {
            println!("Data[{}]: {}", x, temp[&x]);
        }
    }

    println!("===============================");

    {
        let mut temp = DATA.get().unwrap().lock().unwrap();

        temp.insert(0, 1);
        temp.insert(1, 2);
        temp.insert(2, 3);
        temp.insert(3, 4);

        for x in 0..4usize {
            println!("Data[{}]: {}", x, temp[&x]);
        }
    }
}
