#[derive(Deserialize)]
pub struct SpriteSpec {
    pub image: String,
    pub frames: Vec<[u32; 4]>,
}
