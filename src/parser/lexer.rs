pub enum Token {
    Ident(String),
    Kywrd(Keyword),
    Symbl(char),
}

pub enum Keyword {
    If,
    Else,
    Def,
    Del,
    None,
    True,
    False,
}

