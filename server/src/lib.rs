use spacetimedb::{reducer, table, Identity, ReducerContext, Table};

#[table(name = client,public)]
pub struct StdbClient {
    #[primary_key]
    client_id: Identity,
    name: Option<String>,
    online: bool
}

#[table(name = vector2,public)]
pub struct StdbVector2 {
    x: f32,
    y: f32
}

#[reducer]
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.client().client_id().find(ctx.sender) {
        ctx.db.client().client_id().update(StdbClient {
            online: true,
            ..user
        });
    } else {
        ctx.db.client().insert(StdbClient {
            name: None,
            client_id: ctx.sender,
            online: true,
        });
    }
}

#[reducer]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.client().client_id().find(ctx.sender) {
        ctx.db.client().client_id().update(StdbClient { online: false, ..user });
    } else {
        log::warn!("Disconnect event for unknown client: {:?}", ctx.sender);
    }
}