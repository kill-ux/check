#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb < 2 {
        return None;
    }
    for a in 2..nb {
        if nb % a == 0 {
            if nb % 2 == 0 {
                return Some(Err(PrimeErr::Even));
            } else {
                return Some(Err(PrimeErr::Divider(a)));
            }
        }
    }
    Some(Ok(nb))
}
