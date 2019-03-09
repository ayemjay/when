#[derive(Debug, Clone, PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, PartialEq)]
pub enum When {
    This,
    Last,
    Past,
    Next,
    Now,
    Today,
    Tonight,
    Tomorrow,
    Yesterday,
    AM,
    PM,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Week,
    Weekday(Weekday),
    When(When),
    Hour(usize),
}

// This enum adds priority value to token, tokens with smaller priority numbers are
// being parsed first
#[derive(Debug, Clone, PartialEq)]
pub enum PToken {
    None,
    Stub,
    // PToken consists of Token and priority of type isize
    PToken(Token, isize)
}
