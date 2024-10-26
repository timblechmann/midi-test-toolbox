use crate::utils::{loop_until_sigint, resolve_input_port, resolve_output_port};
use midir::{Ignore, MidiInput, MidiOutput, MidiOutputConnection};
use wmidi::MidiMessage;

fn echo_message(connection: &mut MidiOutputConnection, message: &[u8], print: bool) {
    if print {
        match MidiMessage::from_bytes(message) {
            Ok(message) => println!("Received: {:?}", message),
            Err(error) => println!("Cannot decode midi message: {error}"),
        }
    }

    connection
        .send(message)
        .unwrap_or_else(|e| println!("MidiIO error: {}", e));
}

pub fn echo(
    input_device: &String,
    output_device: &String,
    print: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut midi_in = MidiInput::new("midi-toolbox input")?;
    let midi_out = MidiOutput::new("midi-toolbox output")?;
    midi_in.ignore(Ignore::None);

    let out_port = resolve_output_port(&midi_out, output_device)?;
    let in_port = resolve_input_port(&midi_in, input_device)?;
    let mut out_connection = midi_out.connect(&out_port, "Echo output")?;

    let _in_connection = midi_in.connect(
        &in_port,
        "MidiToolbox input",
        move |_stamp, message, _| echo_message(&mut out_connection, message, print),
        (),
    );
    loop_until_sigint()
}
