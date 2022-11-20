use rspotify::{clients::pagination::Paginator, ClientResult};
use futures::stream::TryStreamExt;

pub async fn fetch_all<'a, T>(paginator: Paginator<'a, ClientResult<T>>) -> Vec<T> {
        paginator.try_collect::<Vec<_>>().await.unwrap()
}