use crate::utils::{loop_until_sigint, resolve_input_port};
use midir::{Ignore, MidiInput};

fn echo_message(_timestamp: u64, message: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message = wmidi::MidiMessage::try_from(message)?;
    println!("Received {:#?}", message);
    Ok(())
}

pub fn dump(input_device: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut midi_in = MidiInput::new("midi-toolbox input")?;
    midi_in.ignore(Ignore::None);

    let in_port = resolve_input_port(&midi_in, input_device)?;

    let _connection = midi_in.connect(
        &in_port,
        "MidiToolbox input",
        |stamp, message, _| echo_message(stamp, message).expect("Message parse error"),
        (),
    );

    loop_until_sigint()
}
