mod stream_helpers;
use num_complex::Complex32;
use soapysdr::Device;
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::stream_helpers::{init_rx, read_chunk};

#[derive(Parser)]
struct Args {
    // SDR Device to Interface with
    #[arg(short, long)]
    device: String,

    // defaults to 30.0db
    #[arg(short, long, default_value_t = 30.0)]
    gain: f64,
    
    // list of center frequencies delim by comma
    #[arg(long, value_delimiter = ',')]
    freqs: Vec<f64>,

    // list of channels delim by comma
    #[arg(long, value_delimiter = ',')]
    channels: Vec<usize>,

    // list of sample rates delim by comma
    #[arg(long, value_delimiter = ',')]
    srates: Vec<f64>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup Ctrlc handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        eprint!("Ctrl-C received, stopping capture...");
        r.store(false, Ordering::SeqCst);
    })?;


    // Capture CLI args
    let args = Args::parse();

    // Create new device
    let mut device: Device = Device::new(args.device.as_str())?;

    // Initialize channel buffers
    let n_channels: usize = args.channels.len();
    let samples_per_read: usize = 8192;

    // Init stream
    let mut stream: soapysdr::RxStream<num_complex::Complex<f32>> = init_rx(&mut device, &args.channels, &args.srates, &args.freqs, args.gain)?;

    // map and collect pipeline
    let mut bufs: Vec<Vec<Complex32>> =
        (0..n_channels)
            .map(|_| vec![Complex32::new(0.0, 0.0); samples_per_read])
            .collect();

    // Activate stream
    stream.activate(None)?;

    // Main loop, stop on CTRL-C
    while running.load(Ordering::SeqCst) {
        let mut slices: Vec<&mut [Complex32]> =
            bufs.iter_mut().map(|b| b.as_mut_slice()).collect();

        let samples_read: usize = read_chunk(&mut stream, &mut slices)?;
        println!("Read {samples_read} samples...");

        // TODO: rustradio processing here
    }

    Ok(())
}
