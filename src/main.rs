use csv::Writer;
use std::error::Error;

fn write_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut wrt = Writer::from_path(path)?;
    wrt.write_record(["Frequency (Hz)", "Occupancy (%)", "File Info"])?;

    let start_freq_mhz = 100.0;
    let end_freq_mhz = 108.00;
    let step_mhz = 12.5;


    let mut current_freq_mhz  = start_freq_mhz;

    while current_freq_mhz <= end_freq_mhz {
        let freq_hz = current_freq_mhz * 1_000_000.0 // convert Mhz to Hz

        let occupancy_percent = 10.00;

        let file_info = format!("File Name: pro_9_1_000002_230411_0101_OC");

        wrt.write_record([
            freq_hz,
            occupancy_percent,
            file_info,
        ])?;

        current_freq_mhz += step_mhz; //step size

        }

    wrt.flush()?;
    Ok(())
}

fn main() {
    let filename = "example.csv";
    match write_csv(filename) {
        Ok(_) => println!("Csv file created"),
        Err(e) => println!("Error creating CSV file: {}", e),
    }
}
