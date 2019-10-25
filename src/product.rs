#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub price:f32,
    pub currency: String,
    pub owner: i32
}
