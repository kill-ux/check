pub fn prev_prime(nbr: u64) -> u64 {
    for x in (2..nbr).rev() {
        let mut nice = true;
        for y in 2..x {
            if x % y == 0 {
                nice = false;
                break;
            }
        }
        if nice {
            return x;
        }
    }
    0
}
