use csv::Writer;
use std::error::Error;
use std::fs::File;

// Try to read
fn write_pro(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_path)?;

    Ok(())
}

fn write_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut wrt = Writer::from_path(path)?;
    wrt.write_record(["Frequency (Hz)", "Occupancy (%)"])?;

    let start_freq_mhz = 30.00 * 1_000_000.00;
    let end_freq_mhz = 47.00 * 1_000_000.00;
    let step_khz = 25.00 * 1_000.00;

    let occupancy_percent = 10.00;

    let occupancy_pencent_str = format!("{}", occupancy_percent);

    let mut current_freq_mhz = start_freq_mhz;

    while current_freq_mhz <= end_freq_mhz {
        let freq_hz = current_freq_mhz;
        let freq_hz_str = format!("{}", freq_hz);

        wrt.write_record([&freq_hz_str, &occupancy_pencent_str])?;

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
