#[derive(PartialEq, Copy, Clone, Debug)]
enum Token {
    Plus,
    Mod,
    Num(i64),
}

use Token::*;

pub fn compute(s: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let tokens = tokenize(s)?;
    parse(&tokens)
}

fn tokenize(s: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let s = s.split(' ');
    s.map(|p| {
        Ok(match p {
            "+" => Plus,
            "%" => Mod,
            n => Num(n.parse()?),
        })
    })
    .collect()
}

fn parse(tokens: &[Token]) -> Result<i64, Box<dyn std::error::Error>> {
    let (first, tail) = match tokens.split_first() {
        None => return Err("Empty string".into()),
        Some((Num(h), t)) => (h, t),
        _ => return Err("string start with an operator".into()),
    };

    tail.chunks(2).fold(Ok(*first), |res, slice| {
        if slice.len() == 2 {
            match (slice[0], slice[1]) {
                (Plus, Num(i)) => res.map(|r| r + i),
                (Mod, Num(i)) => res.map(|r| r % i),
                _ => Err("Two consecutive operator or numbers".into()),
            }
        } else {
            Err("Two consecutive operator or numbers".into())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_basic_test() {
        let a = "42 + 0 % 13";
        assert_eq!(
            vec![Num(42), Plus, Num(0), Mod, Num(13)],
            tokenize(a).unwrap()
        );
    }

    #[test]
    fn tokenize_negative_test() {
        let a = "-42 + 0 % 13";
        assert_eq!(
            vec![Num(-42), Plus, Num(0), Mod, Num(13)],
            tokenize(a).unwrap()
        );
    }

    #[test]
    fn tokenize_fail_test() {
        let a = "-42 + 0 % 13 prout";
        assert!(tokenize(a).is_err());
    }

    #[test]
    fn parse_basic_test() {
        let a = vec![Num(42), Plus, Num(8)];
        assert_eq!(50, parse(&a).unwrap());
        let a = vec![Num(42), Plus, Num(8), Mod, Num(10)];
        assert_eq!(0, parse(&a).unwrap());
        let a = vec![Num(42), Plus, Num(-50)];
        assert_eq!(-8, parse(&a).unwrap());
    }

    #[test]
    fn parse_empty_test() {
        let a = vec![];
        assert!(parse(&a).is_err());
        let a = vec![Plus];
        assert!(parse(&a).is_err());
        let a = vec![Plus, Num(12)];
        assert!(parse(&a).is_err());
        let a = vec![Num(12), Plus, Num(8), Num(3)];
        assert!(parse(&a).is_err());
    }
}
