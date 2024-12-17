use crate::domain::entities::user::User;

#[derive(serde::Deserialize, Debug)]
pub struct UserDTO {
    pub cpf: String,
    pub name: String,
    pub email: String,
    pub group: String
}

impl From<UserDTO> for User {
    fn from(user_dto: UserDTO) -> Self {
        User {
            cpf: user_dto.cpf,
            name: user_dto.name,
            email: user_dto.email,
            group: user_dto.group
        }
    }
}