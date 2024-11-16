use std::{error::Error, fmt};

#[derive(Debug)]
pub struct UserAlreadyExistsError
{
    pub cpf: String,
}

impl UserAlreadyExistsError {
    pub fn new(cpf: String) -> Self {
        UserAlreadyExistsError { cpf }
    }
}

impl fmt::Display for UserAlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User with CPF {} already exists", self.cpf)
    }
}

impl Error for UserAlreadyExistsError {}