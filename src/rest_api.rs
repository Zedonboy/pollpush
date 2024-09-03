use warp::{Filter, Rejection, Reply};
use crate::models::{StockQuote, ErrorResponse};
use crate::stock_data::StockData;
use std::convert::Infallible;
use log::error;

pub fn stock_routes(
    stock_data: StockData,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_stock_quote(stock_data.clone())
        .or(get_all_stocks(stock_data))
}

pub fn get_stock_quote(
    stock_data: StockData,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "stock" / String)
        .and(warp::get())
        .and(with_stock_data(stock_data))
        .and_then(handle_get_stock_quote)
}

pub fn get_all_stocks(
    stock_data: StockData,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "stocks")
        .and(warp::get())
        .and(with_stock_data(stock_data))
        .and_then(handle_get_all_stocks)
}

fn with_stock_data(
    stock_data: StockData,
) -> impl Filter<Extract = (StockData,), Error = Infallible> + Clone {
    warp::any().map(move || stock_data.clone())
}

async fn handle_get_stock_quote(symbol: String, stock_data: StockData) -> Result<impl Reply, Rejection> {
    let data = stock_data.read().await;
    match data.get(&symbol) {
        Some(quote) => Ok(warp::reply::json(quote)),
        None => {
            let error = ErrorResponse { error: format!("Stock {} not found", symbol) };
            Ok(warp::reply::json(&error))
        }
    }
}

async fn handle_get_all_stocks(stock_data: StockData) -> Result<impl Reply, Rejection> {
    let data = stock_data.read().await;
    let stocks: Vec<StockQuote> = data.values().cloned().collect();
    Ok(warp::reply::json(&stocks))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (warp::http::StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (warp::http::StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed".to_string())
    } else {
        error!("unhandled rejection: {:?}", err);
        (warp::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
    };

    let json = warp::reply::json(&ErrorResponse {
        error: message,
    });

    Ok(warp::reply::with_status(json, code))
}