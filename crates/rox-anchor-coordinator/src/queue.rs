//! RO:WHAT — Local coordinator queue model.
//! RO:WHY — Tracks pending review requests with duplicate and capacity checks.
//! RO:INTERACTS — CoordinatorReviewRequest and CoordinatorConfig.
//! RO:INVARIANTS — duplicate operation IDs and full queues fail closed.
//! RO:SECURITY — local memory model only; no persistence, wallet calls, RPC, or settlement.
//! RO:TEST — covered by duplicate and capacity tests.

use crate::{CoordinatorConfig, CoordinatorReviewRequest};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinatorQueueError {
    DuplicateOperation,
    QueueFull,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorQueue {
    config: CoordinatorConfig,
    items: Vec<CoordinatorReviewRequest>,
}

impl CoordinatorQueue {
    pub fn new(config: CoordinatorConfig) -> Self {
        Self {
            config,
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, request: CoordinatorReviewRequest) -> Result<(), CoordinatorQueueError> {
        if self
            .items
            .iter()
            .any(|item| item.package.operation_id == request.package.operation_id)
        {
            return Err(CoordinatorQueueError::DuplicateOperation);
        }

        if self.items.len() >= self.config.max_queue_items {
            return Err(CoordinatorQueueError::QueueFull);
        }

        self.items.push(request);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[CoordinatorReviewRequest] {
        &self.items
    }
}
