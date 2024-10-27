use crate::{extract, load, query_create, query_read, query_update, query_delete, measure_time_and_memory};


fn main() {
    let url = "https://media.githubusercontent.com/media/nickeubank/MIDS_Data/master/World_Development_Indicators/wdi_small_tidy_2015.csv";
    let file_path = "data/wdi.csv";
    let directory = "data";

    measure_time_and_memory("Extract", || extract(url, file_path, directory)).unwrap();
    measure_time_and_memory("Load", || load(file_path)).unwrap();
    measure_time_and_memory("Query Create", query_create).unwrap();
    measure_time_and_memory("Query Read", query_read).unwrap();
    measure_time_and_memory("Query Update", query_update).unwrap();
    measure_time_and_memory("Query Delete", query_delete).unwrap();
}
