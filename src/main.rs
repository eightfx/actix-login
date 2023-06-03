mod auth_token;
mod models;
mod error;
mod crypto;
use crypto::HashPassword;
use auth_token::TokenService;
use uuid::Uuid;
use dotenvy::dotenv;
use sqlx::{MySql, Pool};
use actix_web::{ResponseError,  web, App, HttpResponse, HttpServer, Responder};
use actix_web::HttpRequest;
use serde::{Serialize, Deserialize};
use models::User;
#[derive(Deserialize)]
pub struct AuthRequest {
	username: String,
	password: String,
}


#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
	username: String,
	password: String,
	email: String,
}


#[derive(Serialize)]
pub struct AuthResponse {
	token: String,
}

pub async fn register(web::Json(info): web::Json<RegisterRequest>, db_pool: web::Data<Pool<MySql>>) -> impl Responder {
	let hashed_password = User::hash_password(&info.password);
	let user_id = Uuid::new_v4();
	let result = sqlx::query!(
		"INSERT INTO Users (user_id, username, email, password_hash) VALUES (?, ?, ?, ?)",
		user_id.to_string(),
		&info.username,
		&info.email,
		hashed_password
	)
		.execute(&**db_pool)
		.await;

	match result {
		Ok(_) => HttpResponse::Created().finish(),
		Err(e) => {
			if let sqlx::Error::Database(db_err) = &e {
				if db_err.message().contains("Duplicate entry") {
					return HttpResponse::Conflict().body("Username or email already taken");
				}
			}
			HttpResponse::InternalServerError().body(format!("Database error: {:?}", e))
		},
	}
}

pub async fn get_username(db_pool: web::Data<Pool<MySql>>, req: HttpRequest) -> impl Responder {
	// Check for the Authorization header
	let user_result = TokenService::get_user(&db_pool, &req);
	match user_result.await {
		Ok(user) => HttpResponse::Ok().json(user.username),
		Err(e) => e.error_response(),

	}
}

pub async fn login(web::Json(info): web::Json<AuthRequest>, db_pool: web::Data<Pool<MySql>>) -> impl Responder {
	let user_result = sqlx::query_as::<_, User>("select * from users where username = ?")
		.bind(&info.username)
		.fetch_one(&**db_pool)
		.await;

	match user_result {
		Ok(user) => {
			if user.verify_password(&info.password){
				let token_result = TokenService::new().create_jwt(&user.user_id);
				match token_result {
					Ok(token) => HttpResponse::Ok().json(AuthResponse { token }),
					Err(_) => HttpResponse::InternalServerError().body("Internal Error"),
				}
			} else {
				HttpResponse::Unauthorized().body("Invalid username or password")
			}
		},
		Err(e) => {
			println!("{:?}", e);
			HttpResponse::Unauthorized().body("Invalid username or password")
		},
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	env_logger::init();
	// Database pool creation...

	let pool = sqlx::mysql::MySqlPoolOptions::new()
		.max_connections(5)
		.connect(&std::env::var("DATABASE_URL").unwrap())
		.await.unwrap();


	HttpServer::new(move || {
		App::new()
		// Here you would normally set up your database pool in the application data
			.app_data(web::Data::new(pool.clone()))
			.route("/api/auth/login", web::post().to(login))
			.route("/api/auth/register", web::post().to(register))
			.route("/get_username", web::get().to(get_username))
	})
		.bind("0.0.0.0:8080")?
		.run()
		.await
}

