#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>,
) -> Outfit {
    let (jacket, hat) = match (formality_level, invitation_message) {
        (Some(num), Ok(_)) => {
            if num > 0 {
                (Jacket::White, Hat::Fedora)
            } else {
                (Jacket::Black, Hat::Fedora)
            }
        }
        (Some(num), _) => {
            if num > 0 {
                (Jacket::White, Hat::Snapback)
            } else {
                (Jacket::Black, Hat::Snapback)
            }
        }
        (None, Ok(_)) => (Jacket::Flowers, Hat::Fedora),

        _ => (Jacket::Flowers, Hat::Baseball),
    };

    Outfit { jacket, hat }
}
