use sqlx::{FromRow, postgres::PgPoolOptions, PgPool};
use crate::settings::{DB_USER, DB_PASSWORD, DB_HOST, DB_PORT, DB_NAME};
#[derive(FromRow)]
pub struct Settings
{
    pub active: bool,
    pub person: bool,
    pub bicycle: bool,
    pub car: bool,
    pub motorcycle: bool,
    pub airplane: bool,
    pub bus: bool,
    pub train: bool,
    pub truck: bool,
    pub boat: bool,
    pub traffic_light: bool,
    pub fire_hydrant: bool,
    pub stop_sign: bool,
    pub parking_meter: bool,
    pub bench: bool,
    pub bird: bool,
    pub cat: bool,
    pub dog: bool,
    pub horse: bool,
    pub sheep: bool,
    pub cow: bool,
    pub elephant: bool,
    pub bear: bool,
    pub zebra: bool,
    pub giraffe: bool,
    pub backpack: bool,
    pub umbrella: bool,
    pub handbag: bool,
    pub tie: bool,
    pub suitcase: bool,
    pub frisbee: bool,
    pub skis: bool,
    pub snowboard: bool,
    pub sports_ball: bool,
    pub kite: bool,
    pub baseball_bat: bool,
    pub baseball_glove: bool,
    pub skateboard: bool,
    pub surfboard: bool,
    pub tennis_racket: bool,
    pub bottle: bool,
    pub wine_glass: bool,
    pub cup: bool,
    pub fork: bool,
    pub knife: bool,
    pub spoon: bool,
    pub bowl: bool,
    pub banana: bool,
    pub apple: bool,
    pub sandwich: bool,
    pub orange: bool,
    pub broccoli: bool,
    pub carrot: bool,
    pub hot_dog: bool,
    pub pizza: bool,
    pub donut: bool,
    pub cake: bool,
    pub chair: bool,
    pub couch: bool,
    pub potted_plant: bool,
    pub bed: bool,
    pub dining_table: bool,
    pub toilet: bool,
    pub tv: bool,
    pub laptop: bool,
    pub mouse: bool,
    pub remote: bool,
    pub keyboard: bool,
    pub cell_phone: bool,
    pub microwave: bool,
    pub oven: bool,
    pub toaster: bool,
    pub sink: bool,
    pub refrigerator: bool,
    pub book: bool,
    pub clock: bool,
    pub vase: bool,
    pub scissors: bool,
    pub teddy_bear: bool,
    pub hair_drier: bool,
    pub toothbrush: bool
}


pub async fn create_connection_pool() -> Result<PgPool, sqlx::Error>
{
    let db_url =  format!(
        "postgres://{user}:{passwd}@{host}:{port}/{name}",
        user = DB_USER,
        passwd = DB_PASSWORD,
        host = DB_HOST,
        port = DB_PORT,
        name = DB_NAME
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;

    Ok(pool)
}


pub async fn get_settings(pool: &PgPool) -> Result<Settings, sqlx::Error>
{
    let settings = sqlx::query_as::<_, Settings>(
            r#"SELECT * from picamera_recordingsettings limit 1"#,
        ).fetch_one(pool)
        .await?;
    
    Ok(settings)
}

