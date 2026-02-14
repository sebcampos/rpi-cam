use video_cam::db;
use anyhow::Result;

#[tokio::test]
async fn test_get_settings() -> Result<()> 
{
    let db_pool  =  db::create_connection_pool().await?;
    let settings =  db::get_settings(&db_pool).await?;
    print!("{}", settings.active);
    
    Ok(())
}
