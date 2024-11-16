pub struct UserGroup;

impl UserGroup {
  pub const UNKNOWN: &'static str = "unknown";
  pub const CUSTOMER: &'static str = "customer";
  pub const ADMINISTRATOR: &'static str = "administrator";
  pub const POINT_OF_SALE: &'static str = "point_of_sale";

  pub fn validate(group: &str) -> bool {
    match group {
      UserGroup::UNKNOWN => false,
      UserGroup::CUSTOMER => true,
      UserGroup::ADMINISTRATOR => true,
      UserGroup::POINT_OF_SALE => true,
      _ => false
    }
  }
}