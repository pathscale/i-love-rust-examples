use model::endpoint::*;
use model::types::*;

pub fn endpoint_localdb_select() -> EndpointSchema {
    EndpointSchema::new(
			"",
			40010,
			vec![
				Field::new("statements", Type::String),
				Field::new("tokens", Type::String)
			],
			vec![Field::new(
				"rows",
				Type::String,
			)],
	)
}
pub fn get_localdb_endpoints() -> Vec<EndpointSchema> {
    vec![endpoint_localdb_select()]
}
