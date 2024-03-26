use std::{fs,error::Error, io, process};


pub fn row_builder()->Result<(),Box<dyn Error>>{
    let raw_file = "src/weather_stations.csv";
    let output_file = "src/obr.csv";

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(raw_file)?;
    
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(output_file)?;
    
    for (idx,result) in reader.records().enumerate(){
        let record = result?;
        for _ in 0..22411{
            writer.write_record(&record)?;
        }
        let percent = 100.0 * (idx as f64) / 44691f64; 
        println!("{percent:.1}");
    }
    writer.flush()?;
    Ok(())

    }