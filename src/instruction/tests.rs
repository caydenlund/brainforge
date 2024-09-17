//! Unit tests for the `instruction` module
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

#[cfg(test)]
use super::{Instr, Instruction};

#[test]
fn parse_instrs_pos_left() {
    let source = "<".as_bytes();
    let expected = vec![Instr::Left];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "<<".as_bytes();
    let expected = vec![Instr::Left, Instr::Left];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "<<<".as_bytes();
    let expected = vec![Instr::Left, Instr::Left, Instr::Left];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "<<<".as_bytes();
    let expected = vec![Instr::Left, Instr::Left, Instr::Left];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "<A<<A".as_bytes();
    let expected = vec![Instr::Left, Instr::Left, Instr::Left];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_right() {
    let source = ">".as_bytes();
    let expected = vec![Instr::Right];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = ">>".as_bytes();
    let expected = vec![Instr::Right, Instr::Right];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = ">>>".as_bytes();
    let expected = vec![Instr::Right, Instr::Right, Instr::Right];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = ">A>>A".as_bytes();
    let expected = vec![Instr::Right, Instr::Right, Instr::Right];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_decr() {
    let source = "-".as_bytes();
    let expected = vec![Instr::Decr];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "--".as_bytes();
    let expected = vec![Instr::Decr, Instr::Decr];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "---".as_bytes();
    let expected = vec![Instr::Decr, Instr::Decr, Instr::Decr];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "-A--A".as_bytes();
    let expected = vec![Instr::Decr, Instr::Decr, Instr::Decr];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_incr() {
    let source = "+".as_bytes();
    let expected = vec![Instr::Incr];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "++".as_bytes();
    let expected = vec![Instr::Incr, Instr::Incr];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "+++".as_bytes();
    let expected = vec![Instr::Incr, Instr::Incr, Instr::Incr];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "+A++A".as_bytes();
    let expected = vec![Instr::Incr, Instr::Incr, Instr::Incr];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_read() {
    let source = ",".as_bytes();
    let expected = vec![Instr::Read];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = ",,".as_bytes();
    let expected = vec![Instr::Read, Instr::Read];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = ",,,".as_bytes();
    let expected = vec![Instr::Read, Instr::Read, Instr::Read];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = ",A,,A".as_bytes();
    let expected = vec![Instr::Read, Instr::Read, Instr::Read];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_mixed() {
    let source = "-+A-+A,A,A..<>A<A>--+A+A".as_bytes();
    let expected = vec![
        Instr::Decr,
        Instr::Incr,
        Instr::Decr,
        Instr::Incr,
        Instr::Read,
        Instr::Read,
        Instr::Write,
        Instr::Write,
        Instr::Left,
        Instr::Right,
        Instr::Left,
        Instr::Right,
        Instr::Decr,
        Instr::Decr,
        Instr::Incr,
        Instr::Incr,
    ];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_braces() {
    let source = "[]".as_bytes();
    let expected = vec![Instr::LBrace(1), Instr::RBrace(0)];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "[][]".as_bytes();
    let expected = vec![
        Instr::LBrace(1),
        Instr::RBrace(0),
        Instr::LBrace(3),
        Instr::RBrace(2),
    ];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "[[]]".as_bytes();
    let expected = vec![
        Instr::LBrace(3),
        Instr::LBrace(2),
        Instr::RBrace(1),
        Instr::RBrace(0),
    ];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
    assert_eq!(expected, actual);

    let source = "[.A[,]AA]-+[A]".as_bytes();
    let expected = vec![
        Instr::LBrace(5),
        Instr::Write,
        Instr::LBrace(4),
        Instr::Read,
        Instr::RBrace(2),
        Instr::RBrace(0),
        Instr::Decr,
        Instr::Incr,
        Instr::LBrace(9),
        Instr::RBrace(8),
    ];
    let actual = Instruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<Instr>>();
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
