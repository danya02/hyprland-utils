use futures_lite::stream::StreamExt;
use hyprland::{
    data::Workspaces,
    event_listener::{Event, EventStream},
    shared::HyprData,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Workspace {
    name: String,
    id: i32,
    active: bool,
}

#[tokio::main]
pub async fn workspaces() -> anyhow::Result<()> {
    // First time fetch
    let data = Workspaces::get_async().await.unwrap();
    let mut workspaces = data
        .iter()
        .filter(|x| x.windows > 0)
        .map(|x| Workspace {
            name: x.name.clone(),
            id: x.id,
            active: x.id == 1,
        })
        .collect::<Vec<Workspace>>();
    workspaces.sort_by_key(|v| v.id);
    println!("{}", serde_json::to_string(&workspaces).unwrap());

    let mut stream = EventStream::new();
    while let Some(Ok(event)) = stream.next().await {
        if let Event::WorkspaceChanged(event) = event {
            let data = Workspaces::get_async().await.unwrap();
            let mut workspaces = data
                .iter()
                .filter(|x| x.windows > 0 || event.id == x.id)
                .map(|x| Workspace {
                    name: x.name.clone(),
                    id: x.id,
                    active: event.id == x.id,
                })
                .collect::<Vec<Workspace>>();
            workspaces.sort_by_key(|v| v.id);

            let workspaces = serde_json::to_string(&workspaces).unwrap();
            println!("{}", workspaces);
        }
    }

    Err(anyhow::anyhow!("Event stream closed"))
}
