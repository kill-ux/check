#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Positive {
            return false;
        }

        match (&self.antigen, &other.antigen) {
            (Antigen::AB, _) => true,
            (Antigen::O, Antigen::O) => true,
            (Antigen::A, Antigen::A | Antigen::O) => true,
            (Antigen::B, Antigen::B | Antigen::O) => true,
            _ => false,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut dnr: Vec<Self> = vec![];
        dnr.push(BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        });
        if self.rh_factor == RhFactor::Positive {
            dnr.push(BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            });
            if self.antigen != Antigen::O {
                dnr.push(self.clone());
                dnr.push(BloodType {
                    antigen: self.antigen.clone(),
                    rh_factor: RhFactor::Negative,
                });
            }
            if self.antigen == Antigen::AB {
                dnr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive,
                });
                dnr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Negative,
                });
                dnr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive,
                });
                dnr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Negative,
                });
            }
        } else {
            if self.antigen != Antigen::O {
                dnr.push(self.clone());
            }
            if self.antigen == Antigen::AB {
                dnr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Negative,
                });
                dnr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Negative,
                });
            }
        }

        dnr
    }

    pub fn recipients(&self) -> Vec<Self> {
        let mut dnr: Vec<Self> = vec![];
        dnr.push(BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        });
        if self.rh_factor == RhFactor::Negative {
            dnr.push(BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            });
            if self.antigen != Antigen::AB {
                dnr.push(self.clone());
                dnr.push(BloodType {
                    antigen: self.antigen.clone(),
                    rh_factor: RhFactor::Positive,
                });
            }
            if self.antigen == Antigen::O {
                dnr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive,
                });
                dnr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Negative,
                });
                dnr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive,
                });
                dnr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Negative,
                });
            }
        } else {
            if self.antigen != Antigen::AB {
                dnr.push(self.clone());
            }
            if self.antigen == Antigen::O {
                dnr.push(BloodType {
                    antigen: Antigen::A,
                    rh_factor: RhFactor::Positive,
                });
                dnr.push(BloodType {
                    antigen: Antigen::B,
                    rh_factor: RhFactor::Positive,
                });
            }
        }

        dnr
    }
}
