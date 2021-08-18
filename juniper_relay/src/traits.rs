use super::*;
use juniper::{
    marker::IsOutputType, meta::MetaType, Arguments, DefaultScalarValue, ExecutionResult, Executor,
    GraphQLType, GraphQLValue, GraphQLValueAsync, Registry, ScalarValue,
};

impl<N, S> GraphQLType<S> for RelayConnectionEdge<N>
where
    N: GraphQLType<S> + RelayConnectionNode,
    N::Context: juniper::Context,
    S: ScalarValue,
{
    fn name(_info: &<N as GraphQLValue<S>>::TypeInfo) -> Option<&str> {
        Some(N::edge_type_name())
    }

    fn meta<'r>(
        info: &<N as GraphQLValue<S>>::TypeInfo,
        registry: &mut Registry<'r, S>,
    ) -> MetaType<'r, S>
    where
        DefaultScalarValue: 'r,
    {
        let fields = &[
            registry.field::<&N>("node", info),
            registry.field::<&String>("cursor", &()),
        ];
        registry.build_object_type::<Self>(info, fields).into_meta()
    }
}

impl<N, S> GraphQLValue<S> for RelayConnectionEdge<N>
where
    N: GraphQLType<S> + RelayConnectionNode,
    N::Context: juniper::Context,
    S: ScalarValue,
{
    type Context = N::Context;
    type TypeInfo = <N as GraphQLValue<S>>::TypeInfo;

    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
        <Self as GraphQLType<S>>::name(info)
    }

    fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
        self.type_name(info).unwrap_or("ConnectionEdge").to_string()
    }

    fn resolve_field(
        &self,
        info: &Self::TypeInfo,
        field_name: &str,
        _args: &Arguments<S>,
        executor: &Executor<Self::Context, S>,
    ) -> ExecutionResult<S> {
        match field_name {
            "node" => executor.resolve_with_ctx(info, &self.node),
            "cursor" => executor.resolve_with_ctx(&(), &self.cursor),
            _ => panic!("Field {} not found on type RelayConnectionEdge", field_name),
        }
    }
}

impl<N, S> GraphQLValueAsync<S> for RelayConnectionEdge<N>
where
    N: GraphQLType<S> + GraphQLValueAsync<S> + RelayConnectionNode + Sync + Send,
    N::TypeInfo: Sync,
    N::Context: juniper::Context + Sync,
    S: ScalarValue + Send + Sync,
{
    fn resolve_field_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        field_name: &'a str,
        _args: &Arguments<S>,
        executor: &'a Executor<Self::Context, S>,
    ) -> juniper::BoxFuture<'a, ExecutionResult<S>> {
        let f = async move {
            match field_name {
                "node" => executor.resolve_with_ctx_async(info, &self.node).await,
                "cursor" => executor.resolve_with_ctx(&(), &self.cursor),
                _ => panic!("Field {} not found on type RelayConnectionEdge", field_name),
            }
        };
        use ::juniper::futures::future;
        future::FutureExt::boxed(f)
    }
}

impl<N, S> IsOutputType<S> for RelayConnectionEdge<N>
where
    N: GraphQLType<S>,
    S: ScalarValue,
{
}

impl<N, S> GraphQLType<S> for RelayConnection<N>
where
    N: GraphQLType<S> + RelayConnectionNode,
    N::Context: juniper::Context,
    S: ScalarValue,
{
    fn name(_info: &<N as GraphQLValue<S>>::TypeInfo) -> Option<&str> {
        Some(N::connection_type_name())
    }

    fn meta<'r>(
        info: &<N as GraphQLValue<S>>::TypeInfo,
        registry: &mut Registry<'r, S>,
    ) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let fields = &[
            registry.field::<&Vec<RelayConnectionEdge<N>>>("edges", info),
            registry.field::<&Vec<&N>>("nodes", info),
            registry.field::<&RelayConnectionPageInfo>("pageInfo", &()),
        ];
        registry.build_object_type::<Self>(info, fields).into_meta()
    }
}

impl<N, S> GraphQLValue<S> for RelayConnection<N>
where
    N: GraphQLType<S> + RelayConnectionNode,
    N::Context: juniper::Context,
    S: ScalarValue,
{
    type Context = N::Context;
    type TypeInfo = <N as GraphQLValue<S>>::TypeInfo;

    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
        <Self as GraphQLType<S>>::name(info)
    }

    fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
        self.type_name(info).unwrap_or("Connection").to_string()
    }

    fn resolve_field(
        &self,
        info: &Self::TypeInfo,
        field_name: &str,
        _args: &Arguments<S>,
        executor: &Executor<Self::Context, S>,
    ) -> ExecutionResult<S> {
        match field_name {
            "edges" => executor.resolve_with_ctx(info, &self.edges),
            "nodes" => {
                let nodes: Vec<&N> = self.edges.iter().map(|edge| &edge.node).collect();
                executor.resolve_with_ctx(info, &nodes)
            }
            "pageInfo" => executor.resolve_with_ctx(&(), &self.page_info),
            _ => panic!("Field {} not found on type RelayConnectionEdge", field_name),
        }
    }
}

impl<N, S> GraphQLValueAsync<S> for RelayConnection<N>
where
    N: GraphQLType<S> + GraphQLValueAsync<S> + RelayConnectionNode + Sync + Send,
    N::TypeInfo: Sync,
    N::Context: juniper::Context + Sync,
    S: ScalarValue + Send + Sync,
{
    fn resolve_field_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        field_name: &'a str,
        _args: &Arguments<S>,
        executor: &'a Executor<Self::Context, S>,
    ) -> juniper::BoxFuture<'a, ExecutionResult<S>> {
        let f = async move {
            match field_name {
                "edges" => executor.resolve_with_ctx_async(info, &self.edges).await,
                "nodes" => {
                    let nodes: Vec<&N> = self.edges.iter().map(|edge| &edge.node).collect();
                    executor.resolve_with_ctx_async(info, &nodes).await
                }
                "pageInfo" => executor.resolve_with_ctx(&(), &self.page_info),
                _ => panic!("Field {} not found on type RelayConnectionEdge", field_name),
            }
        };
        use ::juniper::futures::future;
        future::FutureExt::boxed(f)
    }
}

impl<N, S> IsOutputType<S> for RelayConnection<N>
where
    N: GraphQLType<S>,
    S: ScalarValue,
{
}
