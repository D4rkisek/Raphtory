use crate::{
    core::{
        entities::nodes::input_node::InputNode,
        storage::timeindex::TimeIndexEntry,
        utils::{errors::GraphError, time::IntoTimeWithFormat},
    },
    db::api::{
        mutation::{
            internal::{InternalAdditionOps, InternalDeletionOps},
            TryIntoInputTime,
        },
        view::StaticGraphViewOps,
    },
};

pub trait DeletionOps: InternalDeletionOps + InternalAdditionOps + Sized {
    fn delete_edge<V: InputNode, T: TryIntoInputTime>(
        &self,
        t: T,
        src: V,
        dst: V,
        layer: Option<&str>,
    ) -> Result<(), GraphError> {
        let ti = TimeIndexEntry::from_input(self, t)?;
        let src_id = self.resolve_node(src.id(), src.id_str());
        let dst_id = self.resolve_node(dst.id(), src.id_str());
        let layer = self.resolve_layer(layer);
        self.internal_delete_edge(ti, src_id, dst_id, layer)
    }

    fn delete_edge_with_custom_time_format<V: InputNode>(
        &self,
        t: &str,
        fmt: &str,
        src: V,
        dst: V,
        layer: Option<&str>,
    ) -> Result<(), GraphError> {
        let time: i64 = t.parse_time(fmt)?;
        self.delete_edge(time, src, dst, layer)
    }
}
