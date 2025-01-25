
#[derive(Component)]
pub enum FoodType {
  FarmProduct,
  AnimalProduct,
}

impl FoodType {
  pub fn to_color(food_type: &FoodType) -> Color {
    match food_type {
      FoodType::FarmProduct => Color::srgb(1.0, 1.0, 0.0), // Yellow
      FoodType::AnimalProduct => Color::srgb(1.0, 0.0, 0.0), // Red
    }
  }
}