use crate::core::downloader::{DownloadRequest, DownloadStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct QueueItem {
    pub id: String,
    pub request: DownloadRequest,
    pub status: DownloadStatus,
}

#[allow(dead_code)]
pub struct DownloadQueue {
    items: Arc<RwLock<HashMap<String, QueueItem>>>,
    next_id: Arc<RwLock<usize>>,
}

#[allow(dead_code)]
impl DownloadQueue {
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn add(&self, request: DownloadRequest) -> String {
        let mut id_counter = self.next_id.write().await;
        let id = format!("download_{}", *id_counter);
        *id_counter += 1;

        let item = QueueItem {
            id: id.clone(),
            request,
            status: DownloadStatus::Pending,
        };

        let mut items = self.items.write().await;
        items.insert(id.clone(), item);

        id
    }

    pub async fn get(&self, id: &str) -> Option<QueueItem> {
        let items = self.items.read().await;
        items.get(id).cloned()
    }

    pub async fn update_status(&self, id: &str, status: DownloadStatus) {
        let mut items = self.items.write().await;
        if let Some(item) = items.get_mut(id) {
            item.status = status;
        }
    }

    pub async fn remove(&self, id: &str) {
        let mut items = self.items.write().await;
        items.remove(id);
    }

    pub async fn list_all(&self) -> Vec<QueueItem> {
        let items = self.items.read().await;
        items.values().cloned().collect()
    }

    pub async fn clear(&self) {
        let mut items = self.items.write().await;
        items.clear();
    }
}

impl Default for DownloadQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::downloader::Platform;

    #[tokio::test]
    async fn test_queue_operations() {
        let queue = DownloadQueue::new();

        let request = DownloadRequest {
            url: "https://www.youtube.com/watch?v=test".to_string(),
            platform: Platform::YouTube,
            output_path: None,
        };

        let id = queue.add(request.clone()).await;
        assert!(!id.is_empty());

        let item = queue.get(&id).await;
        assert!(item.is_some());

        let item = item.unwrap();
        assert_eq!(item.request.url, request.url);

        let all_items = queue.list_all().await;
        assert_eq!(all_items.len(), 1);

        queue.remove(&id).await;
        let item = queue.get(&id).await;
        assert!(item.is_none());
    }
}
