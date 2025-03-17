use spacetimedb::{table, Identity};

#[table(name = client,public)]
pub struct StdbClient {
    #[primary_key]
    client_id: Identity,
    online: bool
}

#[table(name = vector2,public)]
pub struct StdbVector2 {
    x: f32,
    y: f32
}
