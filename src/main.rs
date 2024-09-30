use csv::Writer;
use std::error::Error;

fn write_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut wrt = Writer::from_path(path)?;
    wrt.write_record(["Frequency (Hz)", "Occupancy (%)", "File Info"])?;

    let start_freq_mhz = 30.00 * 1_000_000.00;
    let end_freq_mhz = 47.00 * 1_000_000.00;
    let step_khz = 25.00 * 1_000.00;

    let mut current_freq_mhz = start_freq_mhz;

    while current_freq_mhz <= end_freq_mhz {
        let freq_hz = current_freq_mhz;

        let occupancy_percent = 10.00;

        let file_info = "File Name: pro_9_1_000002_230411_0101_OC"
            .parse()
            .expect("Error");

        let freq_hz_str = format!("{}", freq_hz);
        let occupancy_pencent_str = format!("{}", occupancy_percent);

        wrt.write_record(&[&freq_hz_str, &occupancy_pencent_str, &file_info])?;

        current_freq_mhz += step_khz; //step size
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
