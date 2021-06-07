use super::ID;
use juniper::{
    marker::{IsInputType, IsOutputType},
    meta::MetaType,
    parser::ScalarToken,
    DefaultScalarValue, ExecutionResult, Executor, FromInputValue, GraphQLType, GraphQLValue,
    GraphQLValueAsync, InputValue, ParseScalarResult, ParseScalarValue, Registry, ScalarValue,
    Selection, ToInputValue,
};
use std::convert::TryInto;

impl<S: ScalarValue, const KIND: u64> GraphQLValue<S> for ID<KIND> {
    type Context = ();
    type TypeInfo = ();

    fn type_name<'i>(&self, _: &'i ()) -> Option<&'i str> {
        Some("ID")
    }

    fn resolve(
        &self,
        _: &(),
        _: Option<&[Selection<S>]>,
        _: &Executor<Self::Context, S>,
    ) -> ExecutionResult<S> {
        Ok(juniper::Value::scalar(self.to_string()))
    }
}

impl<S, const KIND: u64> GraphQLValueAsync<S> for ID<KIND>
where
    Self::TypeInfo: Sync,
    Self::Context: Sync,
    S: ScalarValue + Send + Sync,
{
    fn resolve_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        selection_set: Option<&'a [Selection<S>]>,
        executor: &'a Executor<Self::Context, S>,
    ) -> juniper::BoxFuture<'a, ExecutionResult<S>> {
        use juniper::futures::future;
        let v = juniper::GraphQLValue::resolve(self, info, selection_set, executor);
        Box::pin(future::ready(v))
    }
}

impl<S: ScalarValue, const KIND: u64> GraphQLType<S> for ID<KIND> {
    fn name(_: &()) -> Option<&'static str> {
        Some("ID")
    }

    fn meta<'r>(_: &(), registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        DefaultScalarValue: 'r,
    {
        registry.build_scalar_type::<ID<KIND>>(&()).into_meta()
    }
}

impl<S: ScalarValue, const KIND: u64> FromInputValue<S> for ID<KIND> {
    fn from_input_value(v: &InputValue<S>) -> Option<Self> {
        v.as_string_value().and_then(|s: &str| s.try_into().ok())
    }
}

impl<S: ScalarValue, const KIND: u64> ParseScalarValue<S> for ID<KIND> {
    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}

impl<S: ScalarValue, const KIND: u64> ToInputValue<S> for ID<KIND> {
    fn to_input_value(&self) -> InputValue<S> {
        self.as_str().to_input_value()
    }
}

impl<S: ScalarValue, const KIND: u64> IsInputType<S> for ID<KIND> {}
impl<S: ScalarValue, const KIND: u64> IsOutputType<S> for ID<KIND> {}
