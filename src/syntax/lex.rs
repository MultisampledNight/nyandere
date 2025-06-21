use logos::Logos;

use crate::aux::Stack;

#[derive(Stack!, Logos)]
pub enum Token<'src> {
    // commands
    #[token("create")]
    Create,
    #[token("pay")]
    Pay,
    #[token("deliver")]
    Deliver,
    #[token("balance")]
    Balance,

    // punctuation
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[regex(r"\r?\n")]
    Newline,
    #[regex(r"eur(os)?|EUR|€", priority = 10)]
    SignEuro,
    #[regex(r"cents?|ct|¢", priority = 10)]
    SignCent,

    #[regex(r"[+-]?\d+\.\d{2}")]
    Decimal(&'src str),
    #[regex(r"[+-]?\d+")]
    Integer(&'src str),
    /// See <https://www.unicode.org/reports/tr31/#R1>, very backwards-compatible.
    #[regex(r"\p{ID_Start}[\p{ID_Continue}-]*")]
    Ident(&'src str),

    #[regex(r"#[^\r\n]*", logos::skip)]
    Comment,
    #[regex(r"[ \t]+")]
    Whitespace,
}
