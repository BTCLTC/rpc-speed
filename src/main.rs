use async_std::task;
use async_timer::{self, Interval};
use chrono::Local;
use isahc::AsyncReadResponseExt;
use prettytable::{format::Alignment, row, Cell, Row, Table};
use serde_json::from_reader;

use std::{fs::File, path::Path, time::Duration};

use state::{JsonRpc, RpcResponse, TableData};

mod state;

const TIMER_INTERVAL: u64 = 30;

fn main() {
    let json_file_path = Path::new("./rpc.json");
    let file_result = File::open(json_file_path);
    if let Ok(file) = file_result {
        let rpc: Vec<JsonRpc> = from_reader(file).unwrap();
        if rpc.len() > 0 {
            let mut rpc_vec = rpc.clone();
            let mut request_count: u64 = 0;
            let mut interval =
                async_timer::Interval::platform_new(Duration::from_secs(TIMER_INTERVAL));
            task::block_on(init_interval(
                &mut rpc_vec,
                &mut interval,
                &mut request_count,
            ));
        }
    }
}

async fn init_interval(
    rpc_vec: &mut Vec<JsonRpc>,
    interval: &mut Interval,
    request_count: &mut u64,
) {
    loop {
        get_data(rpc_vec, request_count).await;
        interval.as_mut().await;
    }
}

async fn get_data(rpc_vec: &mut Vec<JsonRpc>, request_count: &mut u64) {
    let mut table_data: Vec<TableData> = Vec::new();
    for item in rpc_vec {
        let mut response_time: i64 = 0;
        let mut block_number: i64 = 0;
        let request_before_time = Local::now().timestamp_millis();
        let result = isahc::post_async(
            &item.rpc,
            r#"{
                "id": 1,
                "jsonrpc": "2.0",
                "method": "eth_getBlockByNumber",
                "params": ["latest", false]
            }"#,
        )
        .await;

        if let Ok(mut response) = result {
            let response_json = response.json::<RpcResponse>().await;

            if let Ok(data) = response_json {
                let request_after_time = Local::now().timestamp_millis();
                response_time = request_after_time - request_before_time;
                item.success_count += 1;

                let without_prefix = data.result.number.trim_start_matches("0x");
                let number_result = i64::from_str_radix(without_prefix, 16);

                if let Ok(number) = number_result {
                    block_number = number;
                }
            } else {
                item.failed_count += 1;
            }
        } else {
            item.failed_count += 1;
        }
        table_data.push(TableData {
            name: item.name.clone(),
            success_count: item.success_count,
            failed_count: item.failed_count,
            success_rate: 100,
            response_time,
            block_number,
        })
    }

    *request_count += 1;
    show_table(&table_data, request_count);
}

fn show_table(table_data: &Vec<TableData>, request_count: &mut u64) {
    // Create the table
    let mut table = Table::new();
    table.set_titles(Row::new(vec![Cell::new_align(
        "RPC test speed bot",
        Alignment::CENTER,
    )
    .with_hspan(6)]));
    table.add_row(row![
        "RPC Name",
        "Request Total Count",
        "Request Success Count",
        "Request Failed Count",
        "Success Rate",
        "Response Time (ms)",
        "Latest Block Number",
    ]);

    for item in table_data {
        table.add_row(row![
            item.name,
            request_count,
            item.success_count,
            item.failed_count,
            format!("{:?}%", item.success_rate),
            item.response_time,
            item.block_number,
        ]);
    }

    table.printstd();
}
