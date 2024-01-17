use crate::model::{filters::node_filter::NodeFilter, graph::node::Node};
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields};
use raphtory::{
    core::utils::errors::GraphError,
    db::{api::view::DynamicGraph, graph::nodes::Nodes},
    prelude::*,
};

#[derive(ResolvedObject)]
pub(crate) struct GqlNodes {
    pub(crate) nn: Nodes<'static, DynamicGraph>,
    pub(crate) filter: Option<NodeFilter>,
}

impl GqlNodes {
    fn update<N: Into<Nodes<'static, DynamicGraph>>>(&self, nodes: N) -> Self {
        GqlNodes::new(nodes, self.filter.clone())
    }
}

impl GqlNodes {
    pub(crate) fn new<N: Into<Nodes<'static, DynamicGraph>>>(
        nodes: N,
        filter: Option<NodeFilter>,
    ) -> Self {
        Self {
            nn: nodes.into(),
            filter,
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Node> + '_> {
        let iter = self.nn.iter().map(Node::from);
        match self.filter.as_ref() {
            Some(filter) => Box::new(iter.filter(|n| filter.matches(n))),
            None => Box::new(iter),
        }
    }
}

#[ResolvedObjectFields]
impl GqlNodes {
    ////////////////////////
    // LAYERS AND WINDOWS //
    ////////////////////////

    async fn layers(&self, names: Vec<String>) -> Result<Self, GraphError> {
        self.nn.layer(names).map(|v| self.update(v))
    }

    async fn layer(&self, name: String) -> Result<Self, GraphError> {
        self.nn.layer(name).map(|v| self.update(v))
    }

    async fn window(&self, start: i64, end: i64) -> Self {
        self.update(self.nn.window(start, end))
    }

    async fn at(&self, time: i64) -> Self {
        self.update(self.nn.at(time))
    }

    async fn before(&self, time: i64) -> Self {
        self.update(self.nn.before(time))
    }

    async fn after(&self, time: i64) -> Self {
        self.update(self.nn.after(time))
    }

    async fn shrink_window(&self, start: i64, end: i64) -> Self {
        self.update(self.nn.shrink_window(start, end))
    }

    async fn shrink_start(&self, start: i64) -> Self {
        self.update(self.nn.shrink_start(start))
    }

    async fn shrink_end(&self, end: i64) -> Self {
        self.update(self.nn.shrink_end(end))
    }

    ////////////////////////
    //// TIME QUERIES //////
    ////////////////////////

    async fn start(&self) -> Option<i64> {
        self.nn.start()
    }

    async fn end(&self) -> Option<i64> {
        self.nn.end()
    }

    /////////////////
    //// List ///////
    /////////////////
    async fn count(&self) -> usize {
        self.iter().count()
    }

    async fn page(&self, limit: usize, offset: usize) -> Vec<Node> {
        let start = offset * limit;
        self.iter().skip(start).take(limit).collect()
    }

    async fn list(&self) -> Vec<Node> {
        self.iter().collect()
    }
}
