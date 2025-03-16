use dotenv::dotenv;

#[derive(Clone)]
pub struct PostgresConfig {
    pub url: String,
    pub max_connections: u32
}

#[derive(Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32
}

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_private_key_path: String,
    pub jwt_public_key_path: String,
}

#[derive(Clone)]
pub struct AppConfig {
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let postgres_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
        let postgres_max_connections = std::env::var("MAX_CONNECTIONS")
            .expect("MAX_CONNECTIONS must be set")
            .parse::<u32>()
            .expect("MAX_CONNECTIONS must be a number");

        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let redis_pool_size = std::env::var("REDIS_POOL_SIZE")
            .expect("REDIS_POOL_SIZE must be set")
            .parse::<u32>()
            .expect("REDIS_POOL_SIZE must be a number");

        let jwt_private_key_path = std::env::var("JWT_PRIVATE_KEY_PATH").expect("JWT_PRIVATE_KEY_PATH must be set");
        let jwt_public_key_path = std::env::var("JWT_PUBLIC_KEY_PATH").expect("JWT_PUBLIC_KEY_PATH must be set");

        Self {
            postgres: PostgresConfig {
                url: postgres_url,
                max_connections: postgres_max_connections
            },
            redis: RedisConfig {
                url: redis_url,
                pool_size: redis_pool_size
            },
            auth: AuthConfig {
                jwt_private_key_path,
                jwt_public_key_path
            }
        }
    }
}