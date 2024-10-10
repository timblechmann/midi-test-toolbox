use crate::generator::Generator;
use crate::loopback_timer::LoopbackTimer;
use crate::utils::Sender;
use crate::utils::{resolve_input_port, resolve_output_port};
use midir::{Ignore, MidiInput, MidiOutput};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::signal;
use tokio::time::sleep;
use wmidi::MidiMessage;

pub fn generate_notes(
    note_duration: Duration,
    duration_between_notes: Duration,
    output_device: &String,
    print: bool,
    loopback_timer: Option<Arc<LoopbackTimer>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let midi_out = MidiOutput::new("midi-toolbox output")?;
    let out_port = resolve_output_port(&midi_out, output_device)?;
    let out_connection = midi_out.connect(&out_port, "Midi Test Tool Generator")?;

    let generator = Generator::new(
        note_duration,
        duration_between_notes,
        Sender::Connection(out_connection),
        print,
        loopback_timer,
    );

    let rt = Builder::new_current_thread().enable_all().build()?;

    rt.block_on(async {
        tokio::spawn(async move {
            loop {
                generator.schedule_note().await;
                sleep(generator.note_offset()).await;
            }
        });

        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
    });
    Ok(())
}

pub fn generate_and_analyse(
    note_duration: Duration,
    duration_between_notes: Duration,
    input_device: &String,
    output_device: &String,
    print: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut midi_in = MidiInput::new("midi-toolbox input")?;
    midi_in.ignore(Ignore::None);
    let in_port = resolve_input_port(&midi_in, input_device)?;

    let analyser = LoopbackTimer::new();
    let captured_analyzer = analyser.clone();
    let _in_connection = midi_in.connect(
        &in_port,
        "MidiToolbox input",
        move |_stamp, message: &[u8], _| {
            let midi_msg = MidiMessage::from_bytes(message);
            match midi_msg {
                Ok(midi_msg) => captured_analyzer.process_received_message(&midi_msg),
                Err(_) => println!("Unhandled midi message: {:?}", midi_msg),
            }
        },
        (),
    );

    let _ = generate_notes(
        note_duration,
        duration_between_notes,
        output_device,
        print,
        Some(analyser.clone()),
    );

    analyser.print_analysis();
    Ok(())
}
