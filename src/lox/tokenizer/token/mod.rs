pub mod token_type;

pub struct Token<'a> {
    pub r#type: token_type::Type,
    pub value: &'a str
}