use crate::analysis;
use crate::utils::to_vec;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::SystemTime;

struct LoopbackTimerImpl {
    pending_notes: BTreeMap<heapless::Vec<u8, 8>, SystemTime>,

    latencies: Vec<Duration>,
}

impl LoopbackTimerImpl {
    fn new() -> Self {
        let mut ret = Self {
            pending_notes: Default::default(),
            latencies: Default::default(),
        };
        ret.latencies.reserve(1000000000);
        ret
    }

    fn record_message(&mut self, midi_message: &wmidi::MidiMessage) {
        let now = SystemTime::now();
        self.pending_notes.insert(to_vec(midi_message), now);
    }

    fn process_received_message(&mut self, midi_message: &wmidi::MidiMessage) {
        let now = SystemTime::now();

        let bytes = to_vec(midi_message);

        let item = self.pending_notes.remove(&bytes);
        let insertion_time = match item {
            Some(item) => item,
            None => {
                print!("Unexpected message received: {:?}", &midi_message);
                return;
            }
        };

        let latency = now.duration_since(insertion_time).unwrap();
        self.latencies.push(latency);
    }

    fn print_analysis(&self) -> (Duration, Duration, Duration) {
        let median: Duration = analysis::median(&self.latencies);
        let mean: Duration = analysis::mean(&self.latencies);
        let max: Duration = match self.latencies.iter().max() {
            Some(max) => *max,
            None => Duration::new(0, 0),
        };

        println!("Median: {:#?}, Mean: {:#?}, Max: {:#?}", median, mean, max);
        (median, mean, max)
    }
}

pub struct LoopbackTimer {
    pimpl: Mutex<LoopbackTimerImpl>,
}

impl LoopbackTimer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            pimpl: Mutex::new(LoopbackTimerImpl::new()),
        })
    }

    pub fn record_message(self: &Arc<Self>, midi_message: &wmidi::MidiMessage) {
        self.pimpl.lock().unwrap().record_message(midi_message);
    }

    pub fn process_received_message(self: &Arc<Self>, midi_message: &wmidi::MidiMessage) {
        self.pimpl
            .lock()
            .unwrap()
            .process_received_message(midi_message);
    }

    pub fn print_analysis(self: &Arc<Self>) -> (Duration, Duration, Duration) {
        self.pimpl.lock().unwrap().print_analysis()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use wmidi::Channel::Ch1;
    use wmidi::MidiMessage::NoteOn;
    use wmidi::{Note, Velocity};

    #[test]
    fn test_loopback_timer() {
        let mut timer = crate::loopback_timer::LoopbackTimerImpl::new();
        let noteon = NoteOn(Ch1, Note::A0, Velocity::MAX);

        timer.record_message(&noteon);
        std::thread::sleep(Duration::from_millis(100));
        timer.process_received_message(&noteon);

        assert_eq!(timer.latencies.len(), 1);
        assert_eq!(timer.pending_notes.len(), 0);
        let analysis = timer.print_analysis();
        assert_ne!(analysis.0, Duration::from_secs_f32(0.0));
        assert_ne!(analysis.1, Duration::from_secs_f32(0.0));
    }
}
