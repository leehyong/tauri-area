#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#![allow(non_camel_case_types)]
mod model;

use model::*;
use tokio::*;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use rbatis::rbatis::Rbatis;
use rbatis::crud::{CRUD, CRUDTable};

lazy_static! {
  // Rbatis是线程、协程安全的，运行时的方法是Send+Sync，无需担心线程竞争
   pub static ref RB:Rbatis= Rbatis::new();
}

#[tauri::command]
async fn all_provinces() -> Vec<Province> {
    let w = RB.new_wrapper().eq("code", "52");
    RB.fetch_list_by_wrapper(w).await.unwrap()
}

#[tauri::command]
async fn get_city(code: String) -> Vec<City> {
    let w = RB.new_wrapper().eq("provinceCode", code);
    RB.fetch_list_by_wrapper(w).await.unwrap()
}

#[tauri::command]
async fn get_area(code: String) -> Vec<Area> {
    let w = RB.new_wrapper().eq("cityCode", code);
    RB.fetch_list_by_wrapper(w).await.unwrap()
}

#[tauri::command]
async fn get_street(code: String) -> Vec<Street> {
    let w = RB.new_wrapper().eq("areaCode", code);
    RB.fetch_list_by_wrapper(w).await.unwrap()
}

#[tauri::command]
async fn get_village(code: String) -> Vec<Village> {
    let w = RB.new_wrapper().eq("streetCode", code);
    RB.fetch_list_by_wrapper(w).await.unwrap()
}


#[tokio::main]
async fn main() {
    //启用日志输出，你也可以使用其他日志框架，这个不限定的
    fast_log::init(fast_log::Config::new().file("target/test.log")).unwrap();
    //初始化连接池
    RB.link("sqlite://./db/area.sqlite").await.unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            all_provinces,
            get_city,
            get_area,
            get_street,
            get_village
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

