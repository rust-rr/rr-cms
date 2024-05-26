use axum::response::Response;

pub async fn mw_response_map(res: Response) -> Response {
    println!("->> {:<12} - {:<24}", "RES_MAPPER", "main_response_mapper");
    println!();

    res
}
