#[derive(Clone, Copy)]
pub enum Pattern {
  HalfQuarter = 0,
  Quarter = 1,
  Half = 2,
  ThreeQuarters = 3,
}

pub fn u8_to_pattern(value: u8) -> Option<Pattern> {
  match value {
    0 => Some(Pattern::HalfQuarter),
    1 => Some(Pattern::Quarter),
    2 => Some(Pattern::Half),
    3 => Some(Pattern::ThreeQuarters),
    _ => None,
  }
}

pub fn pattern_to_u8(pattern: Pattern) -> u8 {
  match pattern {
    Pattern::HalfQuarter => 0,
    Pattern::Quarter => 1,
    Pattern::Half => 2,
    Pattern::ThreeQuarters => 3,
  }
}
