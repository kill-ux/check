pub fn next_prime(mut nbr: u64) -> u64 {
    if nbr < 3 {
        return 2;
    }
    loop {
        let mut next = false;
        for a in 2..nbr {
            if nbr % a == 0 {
                next = true;
            }
        }
        if !next {
            return nbr;
        }
        nbr += 1;
    }
}
