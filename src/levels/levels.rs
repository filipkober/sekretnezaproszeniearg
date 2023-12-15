#[derive(Clone)]
pub enum LevelName {
    EightOEightOEight,
    HazardDutyPay,
    Krystle,
    RainbowSix,
    HollywoodBaby,
    WesternUnion,
    Toothless,
    GodLovesYou,
    KnownForIt,
    DrakeEra,
    OutBy16DeadOnTheScene,
    TheFear,
    Tantor,
    Deathcamp,
    Burfict,
    The27Club,
    FreeTheFrail,
    Today
}

pub fn num_to_level_enum(number: usize) -> Option<LevelName> {
    match number {
        0 => Some(LevelName::EightOEightOEight),
        1 => Some(LevelName::HazardDutyPay),
        2 => Some(LevelName::Krystle),
        3 => Some(LevelName::RainbowSix),
        4 => Some(LevelName::HollywoodBaby),
        5 => Some(LevelName::WesternUnion),
        6 => Some(LevelName::Toothless),
        7 => Some(LevelName::GodLovesYou),
        8 => Some(LevelName::KnownForIt),
        9 => Some(LevelName::DrakeEra),
        10 => Some(LevelName::OutBy16DeadOnTheScene),
        11 => Some(LevelName::TheFear),
        12 => Some(LevelName::Tantor),
        13 => Some(LevelName::Deathcamp),
        14 => Some(LevelName::Burfict),
        15 => Some(LevelName::The27Club),
        16 => Some(LevelName::FreeTheFrail),
        17 => Some(LevelName::Today),
        _ => None
    }
}