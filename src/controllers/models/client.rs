use crate::domain::entities::client::Client;

#[derive(serde::Deserialize, Debug)]
pub struct ClientDTO {
    pub cpf: String,
    pub name: String,
    pub email: String
}

impl From<ClientDTO> for Client {
    fn from(client_dto: ClientDTO) -> Self {
        Client {
            cpf: client_dto.cpf,
            name: client_dto.name,
            email: client_dto.email
        }
    }
}