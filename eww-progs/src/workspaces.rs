use hyprland::event_listener::EventStream;
use futures_lite::stream::StreamExt;

pub async fn workspaces() {
    
    let mut stream = EventStream::new();
    while let Some(Ok(event)) = stream.next().await {
        println!("{event:?}");
    }
}
