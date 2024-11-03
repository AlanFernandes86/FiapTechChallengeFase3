use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ClientAlreadyExistsError
{
    pub cpf: String,
}

impl ClientAlreadyExistsError {
    pub fn new(cpf: String) -> Self {
        ClientAlreadyExistsError { cpf }
    }
}

impl fmt::Display for ClientAlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client with CPF {} already exists", self.cpf)
    }
}

impl Error for ClientAlreadyExistsError {}