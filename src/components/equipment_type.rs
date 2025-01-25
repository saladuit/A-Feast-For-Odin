#[derive(Component)] 
pub enum EquipmentType {
  CraftProduct,
  LuxuryGood,
}

impl EquipmentType {
  pub fn to_color(equipment_type: &EquipmentType) -> Color {
    match equipment_type {
      EquipmentType::CraftProduct => Color::srgb(0.0, 1.0, 0.0), // Green
      EquipmentType::LuxuryGood => Color::srgb(0.0, 0.0, 1.0), // Blue
    }
  }
}