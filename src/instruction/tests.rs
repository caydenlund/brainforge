//! Unit tests for the `instruction` module
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

#[cfg(test)]
use super::{BasicInstructionType, BasicInstruction};

#[test]
fn parse_instrs_pos_left() {
    let source = "<".as_bytes();
    let expected = vec![BasicInstructionType::Left];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "<<".as_bytes();
    let expected = vec![BasicInstructionType::Left, BasicInstructionType::Left];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "<<<".as_bytes();
    let expected = vec![BasicInstructionType::Left, BasicInstructionType::Left, BasicInstructionType::Left];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "<<<".as_bytes();
    let expected = vec![BasicInstructionType::Left, BasicInstructionType::Left, BasicInstructionType::Left];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "<A<<A".as_bytes();
    let expected = vec![BasicInstructionType::Left, BasicInstructionType::Left, BasicInstructionType::Left];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_right() {
    let source = ">".as_bytes();
    let expected = vec![BasicInstructionType::Right];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = ">>".as_bytes();
    let expected = vec![BasicInstructionType::Right, BasicInstructionType::Right];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = ">>>".as_bytes();
    let expected = vec![BasicInstructionType::Right, BasicInstructionType::Right, BasicInstructionType::Right];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = ">A>>A".as_bytes();
    let expected = vec![BasicInstructionType::Right, BasicInstructionType::Right, BasicInstructionType::Right];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_decr() {
    let source = "-".as_bytes();
    let expected = vec![BasicInstructionType::Decr];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "--".as_bytes();
    let expected = vec![BasicInstructionType::Decr, BasicInstructionType::Decr];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "---".as_bytes();
    let expected = vec![BasicInstructionType::Decr, BasicInstructionType::Decr, BasicInstructionType::Decr];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "-A--A".as_bytes();
    let expected = vec![BasicInstructionType::Decr, BasicInstructionType::Decr, BasicInstructionType::Decr];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_incr() {
    let source = "+".as_bytes();
    let expected = vec![BasicInstructionType::Incr];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "++".as_bytes();
    let expected = vec![BasicInstructionType::Incr, BasicInstructionType::Incr];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "+++".as_bytes();
    let expected = vec![BasicInstructionType::Incr, BasicInstructionType::Incr, BasicInstructionType::Incr];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "+A++A".as_bytes();
    let expected = vec![BasicInstructionType::Incr, BasicInstructionType::Incr, BasicInstructionType::Incr];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_read() {
    let source = ",".as_bytes();
    let expected = vec![BasicInstructionType::Read];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = ",,".as_bytes();
    let expected = vec![BasicInstructionType::Read, BasicInstructionType::Read];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = ",,,".as_bytes();
    let expected = vec![BasicInstructionType::Read, BasicInstructionType::Read, BasicInstructionType::Read];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = ",A,,A".as_bytes();
    let expected = vec![BasicInstructionType::Read, BasicInstructionType::Read, BasicInstructionType::Read];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_mixed() {
    let source = "-+A-+A,A,A..<>A<A>--+A+A".as_bytes();
    let expected = vec![
        BasicInstructionType::Decr,
        BasicInstructionType::Incr,
        BasicInstructionType::Decr,
        BasicInstructionType::Incr,
        BasicInstructionType::Read,
        BasicInstructionType::Read,
        BasicInstructionType::Write,
        BasicInstructionType::Write,
        BasicInstructionType::Left,
        BasicInstructionType::Right,
        BasicInstructionType::Left,
        BasicInstructionType::Right,
        BasicInstructionType::Decr,
        BasicInstructionType::Decr,
        BasicInstructionType::Incr,
        BasicInstructionType::Incr,
    ];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_pos_braces() {
    let source = "[]".as_bytes();
    let expected = vec![BasicInstructionType::LBrace(1), BasicInstructionType::RBrace(0)];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "[][]".as_bytes();
    let expected = vec![
        BasicInstructionType::LBrace(1),
        BasicInstructionType::RBrace(0),
        BasicInstructionType::LBrace(3),
        BasicInstructionType::RBrace(2),
    ];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "[[]]".as_bytes();
    let expected = vec![
        BasicInstructionType::LBrace(3),
        BasicInstructionType::LBrace(2),
        BasicInstructionType::RBrace(1),
        BasicInstructionType::RBrace(0),
    ];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);

    let source = "[.A[,]AA]-+[A]".as_bytes();
    let expected = vec![
        BasicInstructionType::LBrace(5),
        BasicInstructionType::Write,
        BasicInstructionType::LBrace(4),
        BasicInstructionType::Read,
        BasicInstructionType::RBrace(2),
        BasicInstructionType::RBrace(0),
        BasicInstructionType::Decr,
        BasicInstructionType::Incr,
        BasicInstructionType::LBrace(9),
        BasicInstructionType::RBrace(8),
    ];
    let actual = BasicInstruction::parse_instrs(source).unwrap().iter().map(|instr| instr.instr).collect::<Vec<BasicInstructionType>>();
    assert_eq!(expected, actual);
}

#[test]
fn parse_instrs_neg_unpaired_lbrace_1() {
    let source = "[".as_bytes();
    if let Err(err) = BasicInstruction::parse_instrs(source) {
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
    if let Err(err) = BasicInstruction::parse_instrs(source) {
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
    if let Err(err) = BasicInstruction::parse_instrs(source) {
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
    if let Err(err) = BasicInstruction::parse_instrs(source) {
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
    if let Err(err) = BasicInstruction::parse_instrs(source) {
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
    if let Err(err) = BasicInstruction::parse_instrs(source) {
        match err {
            crate::BFError::ParseError(err) => match err {
                crate::BFParseError::UnmatchedRBrace(idx) => assert_eq!(idx, 2),
                _ => panic!("Wrong parse error type"),
            },
            _ => panic!("Wrong error type"),
        }
    };
}
