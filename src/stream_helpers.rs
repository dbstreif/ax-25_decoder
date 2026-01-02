use soapysdr::{Device, RxStream};
use num_complex::Complex32;

pub fn init_rx(dev: &mut Device, chs: &[usize], sample_rates: &[f64], frequencies: &[f64], gain: f64) ->
Result<RxStream<Complex32>, soapysdr::Error> {
    // Configure RX channel on dev
    for i in 0..chs.len() {
        dev.set_sample_rate(soapysdr::Direction::Rx, chs[i], sample_rates[i])?;
        dev.set_frequency(soapysdr::Direction::Rx, chs[i], frequencies[i], ())?;
        dev.set_gain(soapysdr::Direction::Rx, chs[i], gain)?;
    }

    // start rx stream with yield IQ samples (I + jQ) in-phase plus quadrature
    let stream: RxStream<Complex32> = dev.rx_stream::<Complex32>(chs)?;

    Ok(stream)
}

pub fn read_chunk(stream: &mut RxStream<Complex32>, bufs: &mut [&mut [Complex32]]) -> Result<usize,
soapysdr::Error> {
    /* make sure to call activate on the stream before applying this function, or a stream error
    * will be thrown */
    if !stream.active() {
        return Err(soapysdr::Error { code: (soapysdr::ErrorCode::StreamError), message:
            (String::from("Stream Inactive!")) });
    }

    // Read Stream
    let samples_read: usize = stream.read(bufs, -1)?;
    Ok(samples_read)
}
