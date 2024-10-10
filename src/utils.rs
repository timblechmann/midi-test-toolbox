use midir::{MidiInput, MidiInputPort, MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::error::Error;

pub fn resolve_input_port<'a>(
    midi_in: &'a MidiInput,
    port_name: &'a String,
) -> Result<MidiInputPort, Box<dyn Error>> {
    let in_ports = midi_in.ports();
    for port in in_ports {
        if let Ok(name) = midi_in.port_name(&port) {
            if name == *port_name {
                return Ok(port);
            }
        }
    }
    Err(Box::from("Cannot open input port"))
}

pub fn resolve_output_port<'a>(
    midi_out: &'a MidiOutput,
    port_name: &'a String,
) -> Result<MidiOutputPort, Box<dyn Error>> {
    let in_ports = midi_out.ports();
    for port in in_ports {
        if let Ok(name) = midi_out.port_name(&port) {
            if name == *port_name {
                return Ok(port);
            }
        }
    }
    Err(Box::from("Cannot open output port"))
}

use runtime::Builder;
use tokio::{runtime, signal};

pub fn loop_until_sigint() -> Result<(), Box<dyn Error>> {
    let rt = Builder::new_current_thread().enable_all().build()?;
    rt.block_on(async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
    });

    Ok(())
}

use heapless::Vec;
use wmidi::{MidiMessage, Note};

pub fn all_notes() -> [Note; 128] {
    static ALL_NOTES: [Note; 128] = [
        Note::CMinus1,
        Note::DbMinus1,
        Note::DMinus1,
        Note::EbMinus1,
        Note::EMinus1,
        Note::FMinus1,
        Note::GbMinus1,
        Note::GMinus1,
        Note::AbMinus1,
        Note::AMinus1,
        Note::BbMinus1,
        Note::BMinus1,
        Note::C0,
        Note::Db0,
        Note::D0,
        Note::Eb0,
        Note::E0,
        Note::F0,
        Note::Gb0,
        Note::G0,
        Note::Ab0,
        Note::A0,
        Note::Bb0,
        Note::B0,
        Note::C1,
        Note::Db1,
        Note::D1,
        Note::Eb1,
        Note::E1,
        Note::F1,
        Note::Gb1,
        Note::G1,
        Note::Ab1,
        Note::A1,
        Note::Bb1,
        Note::B1,
        Note::C2,
        Note::Db2,
        Note::D2,
        Note::Eb2,
        Note::E2,
        Note::F2,
        Note::Gb2,
        Note::G2,
        Note::Ab2,
        Note::A2,
        Note::Bb2,
        Note::B2,
        Note::C3,
        Note::Db3,
        Note::D3,
        Note::Eb3,
        Note::E3,
        Note::F3,
        Note::Gb3,
        Note::G3,
        Note::Ab3,
        Note::A3,
        Note::Bb3,
        Note::B3,
        Note::C4,
        Note::Db4,
        Note::D4,
        Note::Eb4,
        Note::E4,
        Note::F4,
        Note::Gb4,
        Note::G4,
        Note::Ab4,
        Note::A4,
        Note::Bb4,
        Note::B4,
        Note::C5,
        Note::Db5,
        Note::D5,
        Note::Eb5,
        Note::E5,
        Note::F5,
        Note::Gb5,
        Note::G5,
        Note::Ab5,
        Note::A5,
        Note::Bb5,
        Note::B5,
        Note::C6,
        Note::Db6,
        Note::D6,
        Note::Eb6,
        Note::E6,
        Note::F6,
        Note::Gb6,
        Note::G6,
        Note::Ab6,
        Note::A6,
        Note::Bb6,
        Note::B6,
        Note::C7,
        Note::Db7,
        Note::D7,
        Note::Eb7,
        Note::E7,
        Note::F7,
        Note::Gb7,
        Note::G7,
        Note::Ab7,
        Note::A7,
        Note::Bb7,
        Note::B7,
        Note::C8,
        Note::Db8,
        Note::D8,
        Note::Eb8,
        Note::E8,
        Note::F8,
        Note::Gb8,
        Note::G8,
        Note::Ab8,
        Note::A8,
        Note::Bb8,
        Note::B8,
        Note::C9,
        Note::Db9,
        Note::D9,
        Note::Eb9,
        Note::E9,
        Note::F9,
        Note::Gb9,
        Note::G9,
    ];

    ALL_NOTES
}

#[allow(dead_code)]
pub enum Sender {
    Function(fn(&MidiMessage) -> ()),
    Connection(MidiOutputConnection),
}

#[warn(dead_code)]

pub fn to_vec(midi_message: &MidiMessage) -> heapless::Vec<u8, 8> {
    let mut ret = Vec::<u8, 8>::new();
    ret.resize(midi_message.bytes_size(), 0).unwrap();
    midi_message.copy_to_slice(&mut ret).unwrap();
    ret
}

#[cfg(target_os = "linux")]
use libc::{self, sched_param, sched_setscheduler};
#[cfg(target_os = "linux")]
pub fn acquire_rt_scheduling() {
    let policy = libc::SCHED_FIFO;
    let priority = 20;
    let param = sched_param {
        sched_priority: priority,
    };

    unsafe {
        let result = sched_setscheduler(0, policy, &param);
        if result != 0 {
            eprintln!("Error setting scheduling policy: {}", result);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use wmidi::Channel::Ch1;
    use wmidi::MidiMessage::NoteOn;
    use wmidi::{MidiMessage, Note, Velocity};

    #[test]
    fn to_vec() {
        let noteon = NoteOn(Ch1, Note::A0, Velocity::MAX);
        let vec = utils::to_vec(&noteon);
        assert_eq!(vec.len(), 3);
        assert_eq!(*vec, [144, 21, 127]);

        assert_eq!(MidiMessage::from_bytes(&vec).ok().unwrap(), noteon);
    }
}
