
use std::{net::SocketAddr, sync::{RwLock, Arc, atomic::AtomicUsize, atomic::Ordering}};
use jwt::Validation;
use serde::{Serialize, Deserialize};
use axum::{ 
    Router, 
    Server, 
    response::{Html, IntoResponse}, 
    routing::{get, post}, 
    Json, 
    http::StatusCode, 
    extract::{FromRequest, RequestParts, TypedHeader, Extension}, 
    headers::{Authorization, authorization::Bearer},
    async_trait 
};
use jsonwebtoken as jwt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: usize,
    pub user_id: usize,
    pub title: String,
}

// matching the post body;
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticle {
    pub title: String
}

// most simplest database;
#[derive(Debug, Default, Clone)]
struct MemDB {
    items: Arc<RwLock<Vec<Article>>>
}

// most simplest jwt claim
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    name: String,
    exp: usize
}

// the login request;
#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    user_id: usize,
    name: String
}

// return the token;
#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String
}

const SECRET: &[u8] = b"secret";
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[tokio::main]
async fn main() {
    let db = MemDB::default();

    let app = Router::new()
        .route("/", get(index))
        .route("/articles", get(get_article)
                            .post(post_article)
                            .layer(Extension(db)))
        .route("/login", post(login));

    let addr = SocketAddr::from(([127,0,0,1], 8888));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn index() -> Html<&'static str> {
    Html("hello axum")
}

async fn db_query(user_id: usize, db: &MemDB) -> Option<Vec<Article>> {
    match db.items.read() {
        Ok(records) => Some(
            records
                .iter()
                .filter(|it| it.user_id == user_id)
                .map(|it| it.clone())
                .collect()
        ),
        Err(_) => None
    }
}


// query article;
async fn get_article(claims: Claims, Extension(db): Extension<MemDB>) -> Result<Json<Vec<Article>>, HttpError> {
    let user_id = claims.id;
    match db_query(user_id, &db).await {
        Some(r) => Ok(Json(r)),
        None => Err(HttpError::Internal)
    }
}

// post an article;
async fn post_article(claims: Claims, Json(postData): Json<CreateArticle>, Extension(db): Extension<MemDB>) -> Result<StatusCode, HttpError> {
    // insert to the data base;
    match db.items.write() {
        Ok(mut guard) => {
            guard.push(Article {
                id: auto_increment(),
                user_id: claims.id,
                title: postData.title,
            });
            Ok(StatusCode::CREATED)
        },
        Err(_) => Err(HttpError::Internal)
    }
}

// login to get the token;
async fn login(Json(login): Json<LoginRequest>) -> Json<Token> {
    let claims = Claims {
        id: login.user_id,
        name: login.name,
        exp: get_epoch() + 24 * 60 * 60
    };

    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(Token { token })

}


#[async_trait]
impl<B> FromRequest<B> for Claims 
where 
    B: Send
{
    type Rejection = HttpError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = 
            TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .map_err(|_| HttpError::Auth)?;

        println!("the incoming token is {:?}", bearer.token());
        let key = jwt::DecodingKey::from_secret(SECRET);
        let token = jwt::decode::<Claims>(bearer.token(), &key, &Validation::default())
            .map_err(|_| HttpError::Auth)?;

        Ok(token.claims)
    }
}


#[derive(Debug)]
enum HttpError {
    Auth,
    Internal
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        };

        (code, msg).into_response()
    }
}


fn get_epoch() -> usize {
    use std::time::SystemTime;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn auto_increment() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}