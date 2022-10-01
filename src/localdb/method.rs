use std::sync::Arc;

use serde::*;
use eyre::*;
use lib::handler::RequestHandler;
use lib::toolbox::*;
use lib::ws::*;
use futures::lock::Mutex;
use reqwest::StatusCode;

use super::db::statements::tokenizer;
use super::db::database::Payload;

pub struct QueryHandler {
	pub db: Arc<Mutex<super::db::database::Database>>,
}

impl RequestHandler for QueryHandler {
	type Request = QueryRequest;
	type Response = QueryResponse;

	fn handle(
		&self,
		toolbox: &Toolbox,
		ctx: RequestContext,
		_conn: Arc<Connection>,
		req: Self::Request,
	) {
		let clone = std::sync::Arc::clone(&self.db);
		toolbox.spawn_response(ctx, async move {
			let tokenization = tokenizer::tokenize_statements(&req.statements, req.tokens);
			let tokenized_statements = match tokenization {
				Ok(t) => t,
				Err(e) => bail!(CustomError::new(
						StatusCode::BAD_REQUEST,
						format!("{:?}", e),
				)),
			};

			let mut guard = clone.lock().await;
			let query_output = guard
				.exec(&tokenized_statements);


			let payloads = match query_output {
				Ok(p) => p,
				Err(e) => bail!(CustomError::new(
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("{:?}", e),
				)),
			};

			Ok(QueryResponse{payloads})
		});
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryRequest {
	statements: String,
	tokens: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResponse {
	payloads: Vec<Payload>,
}
