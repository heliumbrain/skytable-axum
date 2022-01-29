use axum::{routing::{get, post}, http::StatusCode, response::IntoResponse, Json, Router, AddExtensionLayer};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::extract::Extension;
use skytable::actions::Actions;
use skytable::ConnectionBuilder;
use uuid::Uuid;


#[tokio::main]
async fn main() {
	let sky = connect_db().await;

	// build our application with a route
	let app = Router::new()
		// `GET /` goes to `root`
		.route("/", get(root))
		// `POST /users` goes to `create_user`
		.route("/users", post(create_user))
		// Add the ExtensionLayer for Skytable
		.layer(AddExtensionLayer::new(sky));


	// run our app with hyper
	// `axum::Server` is a re-export of `hyper::Server`
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
	"Hello, World!"
}

async fn create_user(
	// Add Skytable conn as a Extension
	Extension(sky): Extension<ConnectionBuilder>,
	// this argument tells axum to parse the request body
	// as JSON into a `CreateUser` type
	Json(payload): Json<CreateUser>,
) -> impl IntoResponse {

	// insert your application logic here
	let user = User {
		id: Uuid::new_v4(),
		username: payload.username,
	};

	// get a connection to Skytable
	let mut con = sky.get_connection().unwrap();

	// save user to Skytable
	con.set(user.id.to_string(), user.username.to_string()).unwrap();


	// this will be converted into a JSON response
	// with a status code of `201 Created`
	(StatusCode::CREATED, Json(user))
}

async fn connect_db() -> skytable::ConnectionBuilder {
	// Handle anything else for setting up the Skytable connection here
	ConnectionBuilder::new().set_host("127.0.0.1".to_string()).set_port(2003)
}


// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
	username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
	id: uuid::Uuid,
	username: String,
}
