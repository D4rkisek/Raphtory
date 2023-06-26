use crate::core::tgraph_shard::errors::GraphError;
use crate::db::view_api::internal::Base;

pub trait InternalDeletionOps {
    fn internal_delete_edge(
        &self,
        t: i64,
        src: u64,
        dst: u64,
        layer: Option<&str>,
    ) -> Result<(), GraphError>;
}

pub trait InheritDeletionOps: Base {}

impl<G: InheritDeletionOps> DelegateDeletionOps for G
where
    G::Base: InternalDeletionOps,
{
    type Internal = G::Base;

    fn graph(&self) -> &Self::Internal {
        self.base()
    }
}

pub trait DelegateDeletionOps {
    type Internal: InternalDeletionOps + ?Sized;

    fn graph(&self) -> &Self::Internal;
}

impl<G: DelegateDeletionOps> InternalDeletionOps for G {
    #[inline(always)]
    fn internal_delete_edge(
        &self,
        t: i64,
        src: u64,
        dst: u64,
        layer: Option<&str>,
    ) -> Result<(), GraphError> {
        self.graph().internal_delete_edge(t, src, dst, layer)
    }
}
