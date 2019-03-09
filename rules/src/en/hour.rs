use time::Duration;
use chrono::prelude::*;

use crate::tokens::{Token, When};
use crate::{rules::RuleResult, TokenDesc};
use crate::consts::HOUR;
use tuple::TupleElements;

use nom::{
    alt, apply, call, many_till, named_args, take, tuple, types::CompleteStr
};

define_num!(hour, (Token::Hour, 0), 0, 12);

define!(
    am:
    [(Token::When(When::AM), 1), "a.m.", 0] |
    [(Token::When(When::AM), 1), "a.", 0] |
    [(Token::When(When::AM), 1), "am", 0]
);

define!(
    pm:
    [(Token::When(When::PM), 1), "p.m.", 0] |
    [(Token::When(When::PM), 1), "p.", 0] |
    [(Token::When(When::PM), 1), "pm", 0]
);

combine!(when => am | pm);

named_args!(parse<'a>(exact_match: bool)<CompleteStr<'a>, (Vec<CompleteStr<'a>>,
                             ( TokenDesc, TokenDesc) )>,

    many_till!(take!(1),
        // time (hours), for example 5am, 6p.m., 4a., 3 p.m.
        tuple!(hour, apply!(when, exact_match))
    )

);

fn make_time(res: &mut RuleResult, local: DateTime<Local>, input: &str) {
    let mut hrs = 0;

    let tokens = res.tokens.as_mut().unwrap();

    for token in tokens {
        match token {
            Token::Hour(n) => {
                hrs = *n;
            },
            Token::When(When::PM) => {
                hrs += 12;
            },
            Token::When(When::AM) => {},
            _ => unreachable!(),
        }
    }

    res.time_shift.as_mut().unwrap().hours = hrs * HOUR;

}

make_interpreter!(indices[0, 1]);

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use crate::tokens::{Token, Weekday as Day, When};
    use crate::MatchBounds;
    use super::interpret;

    fn fixed_time() -> DateTime<Local> {
        Local.ymd(2019, 1, 1).and_hms(0, 0, 0)
    }

    #[test]
    fn test_pm() {
        let mut result = interpret("5pm", false, fixed_time());
        assert_eq!(result.tokens, Some(vec![Token::Hour(5), Token::When(When::PM)]));
        assert_eq!(result.bounds, Some(MatchBounds { start_idx: 0, end_idx: 2 }));
        assert_eq!(result.get_hours(), 61200);

        result = interpret("at 5 pm", false, fixed_time());
        assert_eq!(result.tokens, Some(vec![Token::Hour(5), Token::When(When::PM)]));
        assert_eq!(result.bounds, Some(MatchBounds { start_idx: 3, end_idx: 6 }));
        assert_eq!(result.get_hours(), 61200);

        result = interpret("at 12 p.", false, fixed_time());
        assert_eq!(result.tokens, Some(vec![Token::Hour(12), Token::When(When::PM)]));
        assert_eq!(result.bounds, Some(MatchBounds { start_idx: 3, end_idx: 7 }));
        assert_eq!(result.get_hours(), 86400);
    }

    #[test]
    fn test_am() {
        let mut result = interpret("5am", false, fixed_time());
        assert_eq!(result.tokens, Some(vec![Token::Hour(5), Token::When(When::AM)]));
        assert_eq!(result.bounds, Some(MatchBounds { start_idx: 0, end_idx: 2 }));
        assert_eq!(result.get_hours(), 18000);

        result = interpret("at 5 a.m.", false, fixed_time());
        assert_eq!(result.tokens, Some(vec![Token::Hour(5), Token::When(When::AM)]));
        assert_eq!(result.bounds, Some(MatchBounds { start_idx: 3, end_idx: 8 }));
        assert_eq!(result.get_hours(), 18000);

        result = interpret("at 12 a.", false, fixed_time());
        assert_eq!(result.tokens, Some(vec![Token::Hour(12), Token::When(When::AM)]));
        assert_eq!(result.bounds, Some(MatchBounds { start_idx: 3, end_idx: 7 }));
        assert_eq!(result.get_hours(), 43200);
    }

}