use crate::loopback_timer::LoopbackTimer;
use crate::utils;
use crate::utils::Sender;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::io::Read;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use wmidi::MidiMessage::{NoteOff, NoteOn};
use wmidi::{Channel, MidiMessage, Note, Velocity};

pub struct Generator {
    note_duration: Duration,
    note_offset: Duration,
    channel: Channel,

    active_notes: Mutex<fixedbitset::FixedBitSet>,
    sender: Mutex<Sender>,
    loopback_timer: Option<Arc<LoopbackTimer>>,
    print: bool,
}

impl Generator {
    pub fn new(
        note_duration: Duration,
        note_offset: Duration,
        sender: Sender,
        print: bool,
        loopback_timer: Option<Arc<LoopbackTimer>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            note_duration,
            note_offset,
            channel: Channel::Ch1,
            active_notes: fixedbitset::FixedBitSet::with_capacity(128).into(),
            sender: sender.into(),
            loopback_timer,
            print,
        })
    }

    pub async fn schedule_note(self: &Arc<Self>) {
        let note = match self.make_note().await {
            Some((note, velocity)) => (note, velocity),
            None => return,
        };

        let duration = self.note_duration;
        let cloned_self = self.clone();
        tokio::spawn(async move {
            sleep(duration).await;
            cloned_self
                .send(NoteOff(
                    cloned_self.channel,
                    note.0,
                    Velocity::from_u8_lossy(0),
                ))
                .await;
            cloned_self
                .active_notes
                .lock()
                .await
                .set(note.0 as usize, false);
        });

        self.send(NoteOn(self.channel, note.0, note.1)).await;
    }

    pub async fn make_note(self: &Generator) -> Option<(Note, Velocity)> {
        let velocity: u8 = rand::thread_rng().gen_range(1..127);
        let available_notes = self.available_notes().await;
        if available_notes.is_empty() {
            return None;
        }

        let note = *available_notes.choose(&mut rand::thread_rng()).unwrap();

        self.active_notes.lock().await.set(note as usize, true);

        Some((note, Velocity::from_u8_lossy(velocity)))
    }

    pub async fn available_notes(self: &Generator) -> heapless::Vec<Note, 128> {
        let active_notes = self.active_notes.lock().await;
        let available_notes = utils::all_notes()
            .iter()
            .filter(|&note| !active_notes.contains(*note as usize))
            .cloned()
            .collect();

        available_notes
    }

    async fn send(self: &Generator, msg: MidiMessage<'_>) {
        let mut sender = self.sender.lock().await;

        if let Some(timer) = self.loopback_timer.clone() {
            timer.record_message(&msg);
        }

        if self.print {
            println!("Sending midi message: {:?}", msg);
        }

        match sender.deref_mut() {
            Sender::Function(f) => f(&msg),
            Sender::Connection(c) => {
                let mut data = heapless::Vec::<u8, 16>::new();
                msg.bytes()
                    .for_each(|byte| data.push(byte.unwrap()).unwrap());

                c.send(&data).expect("Sending failed")
            }
        }
    }

    pub fn note_offset(&self) -> Duration {
        self.note_offset
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::Generator;
    use crate::utils::Sender;
    use std::time::Duration;

    #[tokio::test]
    async fn test_make_note() {
        let gen = Generator::new(
            Duration::from_millis(100),
            Duration::from_millis(100),
            Sender::Function(|msg| println!("Sending {:#?}", msg)),
            false,
            None,
        );
        assert_eq!(gen.available_notes().await.len(), 128);
        let note = gen.make_note().await.unwrap().0;
        assert!(!gen.available_notes().await.contains(&note));
        assert!(gen.active_notes.lock().await.contains(note as usize));
    }

    #[tokio::test]
    async fn test_schedule_note() {
        let gen = Generator::new(
            Duration::from_millis(100),
            Duration::from_millis(100),
            Sender::Function(|msg| println!("Sending {:#?}", msg)),
            false,
            None,
        );
        assert_eq!(gen.available_notes().await.len(), 128);
        println!("schedule note");
        gen.schedule_note().await;
        println!("schedule note done");
        tokio::time::sleep(Duration::from_millis(1000)).await;
        assert_eq!(gen.available_notes().await.len(), 128);
    }
}
