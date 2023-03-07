mod http_server;
mod core;

#[actix_web::main]
async fn main(){
    dotenv::dotenv().ok();
    let db_pool = core::create_db_pool(std::env::var("DATABASE_URL").unwrap().as_str()).await;
    http_server::launch(db_pool.clone()).await;
}

pub struct Vec<T> {
    ptr: std::ptr::NonNull<T>,
    cap: usize,
    len: usize
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}