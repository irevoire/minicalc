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
    let (i, tail) = match tokens.split_first() {
        None => return Err("Empty string".into()),
        Some((Num(h), t)) => (h, t),
        _ => return Err("Start with an operator".into()),
    };

    match tail.split_first() {
        None => Ok(*i), // only one number
        Some((Plus, t)) => plus(t, &|n| i + n),
        Some((Mod, t)) => modulo(t, &|n| i % n),
        _ => Err("Start with two consecutive number".into()),
    }
}

fn plus(tokens: &[Token], func: &dyn Fn(i64) -> i64) -> Result<i64, Box<dyn std::error::Error>> {
    let (i, tail) = match tokens.split_first() {
        None => return Err("End with an operator".into()),
        Some((Num(h), t)) => (h, t),
        _ => return Err("Two consecutive operator".into()),
    };

    match tail.split_first() {
        None => Ok(func(*i)), // end of the calculus
        Some((Plus, t)) => Ok(func(plus(t, &|n| i + n)?)),
        Some((Mod, t)) => Ok(func(modulo(t, &|n| i % n)?)),
        _ => Err("Two consecutive number".into()),
    }
}

fn modulo(tokens: &[Token], func: &dyn Fn(i64) -> i64) -> Result<i64, Box<dyn std::error::Error>> {
    let (i, tail) = match tokens.split_first() {
        None => return Err("End with an operator".into()),
        Some((Num(h), t)) => (h, t),
        _ => return Err("Two consecutive operator".into()),
    };

    match tail.split_first() {
        None => Ok(func(*i)), // end of the calculus
        Some((Plus, t)) => Ok(plus(t, &|n| func(*i) + n)?),
        Some((Mod, t)) => Ok(modulo(t, &|n| func(*i) % n)?),
        _ => Err("Two consecutive number".into()),
    }
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
        let a = vec![Num(42), Plus, Num(10), Mod, Num(8)];
        assert_eq!(44, parse(&a).unwrap());
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

    #[test]
    fn real_parse_test() {
        let a = "735926 % 51245 + 735926 % 913";
        assert_eq!(compute(&a).unwrap(), 18_544);
    }
}
