fn main() {
    let mut f_np  = 1;
    let mut f_n = 2;
    let mut sum = f_n;

    while f_n <= 4000000 {
        let temp = f_n;
        f_n += f_np;
        f_np = temp;
        if f_n % 2 == 0 {
            sum += f_n;
        }
    }

    println!("{}", sum);
}