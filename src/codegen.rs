//

use azml::azml::{
    adjunct::{adjunct, adjunct_entity},
    entity::entity,
    value_object::value_object,
};

pub trait CodeGenerator {
    fn generate_entity_codes(
        &self,
        module_name: &String,
        ent: &entity::Entity,
        identifier: &String,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn generate_adjunct_entity_codes(
        &self,
        module_name: &String,
        adj_ent: &adjunct_entity::AdjunctEntity,
        identifier: &String,
        hosts: &Vec<adjunct::AdjunctHost>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn generate_value_object_codes(
        &self,
        module_name: &String,
        vo: &value_object::ValueObject,
        identifier: &String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
