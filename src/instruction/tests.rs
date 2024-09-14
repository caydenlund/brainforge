//! Unit tests for the `instruction` module
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

#[cfg(test)]
use super::Instruction;

#[test]
fn parse_instrs_pos_left() {
    let source = "<".as_bytes();
    let expected = vec![Instruction::Left];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "<<".as_bytes();
    let expected = vec![Instruction::Left, Instruction::Left];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "<<<".as_bytes();
    let expected = vec![Instruction::Left, Instruction::Left, Instruction::Left];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "<<<".as_bytes();
    let expected = vec![Instruction::Left, Instruction::Left, Instruction::Left];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "<A<<A".as_bytes();
    let expected = vec![Instruction::Left, Instruction::Left, Instruction::Left];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_right() {
    let source = ">".as_bytes();
    let expected = vec![Instruction::Right];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = ">>".as_bytes();
    let expected = vec![Instruction::Right, Instruction::Right];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = ">>>".as_bytes();
    let expected = vec![Instruction::Right, Instruction::Right, Instruction::Right];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = ">A>>A".as_bytes();
    let expected = vec![Instruction::Right, Instruction::Right, Instruction::Right];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_decr() {
    let source = "-".as_bytes();
    let expected = vec![Instruction::Decr];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "--".as_bytes();
    let expected = vec![Instruction::Decr, Instruction::Decr];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "---".as_bytes();
    let expected = vec![Instruction::Decr, Instruction::Decr, Instruction::Decr];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "-A--A".as_bytes();
    let expected = vec![Instruction::Decr, Instruction::Decr, Instruction::Decr];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_incr() {
    let source = "+".as_bytes();
    let expected = vec![Instruction::Incr];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "++".as_bytes();
    let expected = vec![Instruction::Incr, Instruction::Incr];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "+++".as_bytes();
    let expected = vec![Instruction::Incr, Instruction::Incr, Instruction::Incr];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "+A++A".as_bytes();
    let expected = vec![Instruction::Incr, Instruction::Incr, Instruction::Incr];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_read() {
    let source = ",".as_bytes();
    let expected = vec![Instruction::Read];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = ",,".as_bytes();
    let expected = vec![Instruction::Read, Instruction::Read];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = ",,,".as_bytes();
    let expected = vec![Instruction::Read, Instruction::Read, Instruction::Read];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = ",A,,A".as_bytes();
    let expected = vec![Instruction::Read, Instruction::Read, Instruction::Read];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_mixed() {
    let source = "-+A-+A,A,A..<>A<A>--+A+A".as_bytes();
    let expected = vec![
        Instruction::Decr,
        Instruction::Incr,
        Instruction::Decr,
        Instruction::Incr,
        Instruction::Read,
        Instruction::Read,
        Instruction::Write,
        Instruction::Write,
        Instruction::Left,
        Instruction::Right,
        Instruction::Left,
        Instruction::Right,
        Instruction::Decr,
        Instruction::Decr,
        Instruction::Incr,
        Instruction::Incr,
    ];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_braces() {
    let source = "[]".as_bytes();
    let expected = vec![Instruction::LBrace(1), Instruction::RBrace(0)];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "[][]".as_bytes();
    let expected = vec![
        Instruction::LBrace(1),
        Instruction::RBrace(0),
        Instruction::LBrace(3),
        Instruction::RBrace(2),
    ];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "[[]]".as_bytes();
    let expected = vec![
        Instruction::LBrace(3),
        Instruction::LBrace(2),
        Instruction::RBrace(1),
        Instruction::RBrace(0),
    ];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);

    let source = "[.A[,]AA]-+[A]".as_bytes();
    let expected = vec![
        Instruction::LBrace(5),
        Instruction::Write,
        Instruction::LBrace(4),
        Instruction::Read,
        Instruction::RBrace(2),
        Instruction::RBrace(0),
        Instruction::Decr,
        Instruction::Incr,
        Instruction::LBrace(9),
        Instruction::RBrace(8),
    ];
    let actual = Instruction::parse_instrs(source).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_neg_unpaired_lbrace_1() {
    let source = "[".as_bytes();
    if let Err(err) = Instruction::parse_instrs(source) {
        match err {
            crate::BFError::ParseError(err) => match err {
                crate::BFParseError::UnmatchedLBrace(idx) => assert_eq!(idx, 0),
                _ => panic!("Wrong parse error type"),
            },
            _ => panic!("Wrong error type"),
        }
    };
}

#[test]
fn parse_instrs_neg_unpaired_lbrace_2() {
    let source = "[[][[]]".as_bytes();
    if let Err(err) = Instruction::parse_instrs(source) {
        match err {
            crate::BFError::ParseError(err) => match err {
                crate::BFParseError::UnmatchedLBrace(idx) => assert_eq!(idx, 0),
                _ => panic!("Wrong parse error type"),
            },
            _ => panic!("Wrong error type"),
        }
    };
}

#[test]
fn parse_instrs_neg_unpaired_lbrace_3() {
    let source = "[[]][[]".as_bytes();
    if let Err(err) = Instruction::parse_instrs(source) {
        match err {
            crate::BFError::ParseError(err) => match err {
                crate::BFParseError::UnmatchedLBrace(idx) => assert_eq!(idx, 4),
                _ => panic!("Wrong parse error type"),
            },
            _ => panic!("Wrong error type"),
        }
    };
}

#[test]
fn parse_instrs_neg_unpaired_rbrace_1() {
    let source = "]".as_bytes();
    if let Err(err) = Instruction::parse_instrs(source) {
        match err {
            crate::BFError::ParseError(err) => match err {
                crate::BFParseError::UnmatchedRBrace(idx) => assert_eq!(idx, 0),
                _ => panic!("Wrong parse error type"),
            },
            _ => panic!("Wrong error type"),
        }
    };
}

#[test]
fn parse_instrs_neg_unpaired_rbrace_2() {
    let source = "[[]][]]".as_bytes();
    if let Err(err) = Instruction::parse_instrs(source) {
        match err {
            crate::BFError::ParseError(err) => match err {
                crate::BFParseError::UnmatchedRBrace(idx) => assert_eq!(idx, 6),
                _ => panic!("Wrong parse error type"),
            },
            _ => panic!("Wrong error type"),
        }
    };
}

#[test]
fn parse_instrs_neg_unpaired_rbrace_3() {
    let source = "[]][[]]".as_bytes();
    if let Err(err) = Instruction::parse_instrs(source) {
        match err {
            crate::BFError::ParseError(err) => match err {
                crate::BFParseError::UnmatchedRBrace(idx) => assert_eq!(idx, 2),
                _ => panic!("Wrong parse error type"),
            },
            _ => panic!("Wrong error type"),
        }
    };
}
