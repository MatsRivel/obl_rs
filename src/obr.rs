use std::collections::HashMap;
use std::time::{Duration, Instant};
use anyhow::{anyhow, Context, Error};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::ParallelBridge;
struct RowData{
    name:String,
    temp:f64
}

impl TryFrom<&str> for RowData{
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self,Error> {
        let info: Vec<&str> = value.split(';').collect::<Vec<&str>>();
        if info.len() != 2{
            return Err(anyhow!("Failed to split line into City"))
        }
        let name = info[0].to_string();
        let temp_str = info[1];
        let temp = temp_str.parse::<f64>().with_context(||anyhow!("Failed to parse string to float!"))?;
        Ok(RowData{name,temp})
    }
}

struct City{
    name:String,
    min:f64,
    max:f64,
    sum:f64,
    count:u64
}

impl City{
    fn add(&mut self, temp:f64 ){
        if self.min > temp{
            self.min = temp;
        }else if self.max < temp{
            self.max = temp;
        }
        self.sum += temp;
        self.count += 1;
    }
    fn get_string(&self)->String{
        let name = &self.name;
        let min = self.min;
        let max = self.max;
        let avg = self.sum / self.count as f64;
        format!("{name}={min:.1}/{avg:.1}/{max:.1}")
    }
}
impl From<RowData> for City{
    fn from(value: RowData) -> Self {
        let name = value.name;
        let val = value.temp;
        Self{name, min:val, max:val, sum:val, count:1}
    }
}

pub fn obr_challenge(file_name:&str)->Result<(),anyhow::Error>{
    let step_size = 10000000.0;
    let final_step = 1000000000.0 / step_size;
    let start = Instant::now();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_name)?;

    
    let map: HashMap<String, City> = reader.records()
        .par_bridge()
            .map(|result| {
                let record = result.unwrap();
                let row = RowData::try_from(record.as_slice()).unwrap();
                (row.name.clone(), row)
        }).fold(HashMap::new(), |mut acc, (row_name, row)| {
            if let Some(city) = acc.get_mut(row_name){
                
            }
        });


    for city in map.values(){
        let row = city.get_string();
        println!("{row}");
    }
    let runtime = start.elapsed();
    let minutes = runtime.as_secs() / 60;
    let seconds = runtime.as_secs() % 60;
    let millies = runtime.subsec_millis();
    println!("Runtime: {minutes} min, {seconds} sec, {millies} ms.");
    Ok(())
}