use midir::{Ignore, MidiInput, MidiOutput};

pub fn list_devices() -> Result<(), Box<dyn std::error::Error>> {
    let mut midi_in = MidiInput::new("Midi Test Tool")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("Midi Test Tool")?;

    println!("Available input ports:");
    for (i, p) in midi_in.ports().iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p)?);
    }

    println!("\nAvailable output ports:");
    for (i, p) in midi_out.ports().iter().enumerate() {
        println!("{}: {}", i, midi_out.port_name(p)?);
    }

    Ok(())
}
