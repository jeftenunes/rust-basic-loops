fn main() {
    array_loop();
    // loopto10();
    // let b = highest(4, 2, 8);
    // println!("{} is the highest", b);
}

fn array_loop() {
    let v = vec![3, 8, 11, 34, 67, 9];
 
    for n in v {
        if n % 2 == 0 {
            break;
        }

        println!("{}", n);
    }
}