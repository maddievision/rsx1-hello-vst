extern crate vst;

pub mod timed_event;
pub mod sample_host;
pub mod schala;
pub mod cmd_sequence;

fn main() {


    // for f in 0..FRAME_COUNT {

    // }

    

    loop {};




    // println!("Writing output to file {}: ", OUTPUT_PATH);

    // fs::create_dir_all(OUTPUT_DIR).expect("could not create output dir");
    // let header = Header::new(
    //     wav::WAV_FORMAT_IEEE_FLOAT,
    //     CHANNELS as u16,
    //     SAMPLE_RATE as u32,
    //     32,
    // );
    // let data = BitDepth::ThirtyTwoFloat(collected);

    // let mut out_file = File::create(Path::new(OUTPUT_PATH)).expect("cannot create output file");
    // wav::write(header, &data, &mut out_file).expect("cannot write to output file");

    println!("Closing instance...");
    drop(plugin);
}
