use egg::{CostFunction, Id, Language, SymbolLang};

pub struct GeoTRSCostFunction;

impl CostFunction<SymbolLang> for GeoTRSCostFunction {
    type Cost = i32;

    fn cost<C>(&mut self, enode: &SymbolLang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let op_cost = match enode.op.as_str() {
            "st_within" => 10,
            "st_disjoint" => 10,
            _ => 0,
        };

        enode.fold(op_cost, |sum, id| sum + costs(id))
    }
}
