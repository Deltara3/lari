use super::Parser;
use crate::lexer::SyntaxKind;


pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(SyntaxKind::Int) | Some(SyntaxKind::Ident) => p.bump(),
        Some(SyntaxKind::Dec) => {
            let op = PreOp::Neg;
            let ((), right_binding_power) = op.binding_power();

            p.bump();

            p.start_node_at(checkpoint, SyntaxKind::PreExpr);
            expr_binding_power(p, right_binding_power);
            p.finish_node();
        },
        Some(SyntaxKind::LParen) => {
            p.bump();
            expr_binding_power(p, 0);

            assert_eq!(p.peek(), Some(SyntaxKind::RParen));
            p.bump();
        }
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Inc) => InOp::Inc,
            Some(SyntaxKind::Dec) => InOp::Dec,
            Some(SyntaxKind::Mul) => InOp::Mul,
            Some(SyntaxKind::Div) => InOp::Div,
            _ => return
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            return;
        }

        p.bump();

        p.start_node_at(checkpoint, SyntaxKind::BinOp);
        expr_binding_power(p, right_binding_power);
        p.finish_node();
    }
}

enum InOp {
    Inc,
    Dec,
    Mul,
    Div
}

impl InOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Inc | Self::Dec => (1, 2),
            Self::Mul | Self::Div => (3, 4)
        }
    }
}

enum PreOp {
    Neg
}

impl PreOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5)
        }
    }
}