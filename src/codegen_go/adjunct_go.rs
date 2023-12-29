//

use std::error;

use crate::convert_case::{Case, Casing};

use azml::azml::{
    adjunct::{adjunct, adjunct_entity, adjunct_prime, adjunct_value},
    symbol,
};

use super::{template::go_unexport, GoCodeGenerator};

//TODO:
// - if there are more than one hosts with the same type ensure that
//   each has an assigned name and ensure to generate code to ensure that
//   they are ordered

impl GoCodeGenerator {
    pub fn generate_adjunct_codes(
        &self,
        module_name: &String,
        adj: &adjunct::Adjunct,
        sym: &symbol::Symbol,
    ) -> Result<(), Box<dyn error::Error>> {
        if let Some(adj_def) = adj
            .definition
            .downcast_ref::<adjunct_entity::AdjunctEntity>()
        {
            self.generate_adjunct_entity_codes(module_name, adj_def, &adj, &sym)?;
            Ok(())
        } else if let Some(adj_def) = adj.definition.downcast_ref::<adjunct_prime::AdjunctPrime>() {
            self.generate_adjunct_prime_codes(module_name, adj_def, &adj, &sym)?;
            Ok(())
        } else if let Some(adj_def) = adj.definition.downcast_ref::<adjunct_value::AdjunctValue>() {
            self.generate_adjunct_value_codes(module_name, adj_def, &adj, &sym)?;
            Ok(())
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Gtmpl)]
pub struct AdjunctHostContext {
    type_name_with_pkg: String,
    bare_type_name: String,
    identifier_name: String,
    name_unexported: String,
    id_name: String,
    db_col_name: String,
}

impl From<&adjunct::AdjunctHost> for AdjunctHostContext {
    fn from(x: &adjunct::AdjunctHost) -> Self {
        let identifier_name = if x.name.is_empty() {
            x.kind.symbol_name.to_owned()
        } else {
            x.name.to_owned()
        };
        Self {
            type_name_with_pkg: if x.kind.package_identifier.is_empty() {
                x.kind.symbol_name.to_owned()
            } else {
                format!("{}.{}", x.kind.package_identifier, x.kind.symbol_name)
            },
            bare_type_name: x.kind.symbol_name.to_owned(),
            identifier_name: identifier_name.to_owned(),
            name_unexported: go_unexport(&identifier_name),
            id_name: format!("{}ID", identifier_name),
            db_col_name: format!("{}_id", identifier_name.to_case(Case::Snake)),
        }
    }
}
