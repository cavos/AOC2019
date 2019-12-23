use super::interpreter::{ExecutionState, IntcodeInterpreter};

struct PhaseSettings {
    settings: [i32; 5],
    state: [usize; 6],
    index: usize,
}

impl PhaseSettings {
    pub fn new(settings: [i32; 5]) -> Self {
        PhaseSettings {
            settings,
            state: [0; 6],
            index: 0,
        }
    }

    fn swap_elements(&mut self, pos_a: usize, pos_b: usize) {
        let tmp = self.settings[pos_a];
        self.settings[pos_a] = self.settings[pos_b];
        self.settings[pos_b] = tmp;
    }
}

impl Iterator for PhaseSettings {
    type Item = [i32; 5];

    fn next(&mut self) -> Option<Self::Item> {
        if self.state[5] == 0 {
            self.state[5] = 1;
            return Some(self.settings.clone());
        }

        while self.index < self.settings.len() {
            if self.state[self.index] < self.index {
                if self.index % 2 == 0 {
                    self.swap_elements(0, self.index);
                } else {
                    self.swap_elements(self.state[self.index], self.index);
                }

                self.state[self.index] += 1;
                self.index = 0;
                return Some(self.settings.clone());
            } else {
                self.state[self.index] = 0;
                self.index += 1;
            }
        }
        None
    }
}

pub fn solve(input_file: &str) {
    let interpreter = IntcodeInterpreter::from_file(input_file);

    let settings = PhaseSettings::new([0, 1, 2, 3, 4]);
    let mut phase_setting = [0i32; 5];
    let mut largest_signal = 0;
    for setting in settings {
        let mut signal = 0;
        for phase in &setting {
            let mut amplifier = interpreter.clone();
            amplifier.set_input(*phase);
            amplifier.set_input(signal);
            amplifier.execute();
            signal = amplifier.get_output().expect("Signal value");
        }

        if signal > largest_signal {
            largest_signal = signal;
            phase_setting = setting;
        }
    }

    println!(
        "Day 07.1: Largest output signal is {} for phase setting {:?}",
        largest_signal, phase_setting
    );
}

pub fn solve_pt2(input_file: &str) {
    let interpreter = IntcodeInterpreter::from_file(input_file);

    let settings = PhaseSettings::new([5, 6, 7, 8, 9]);
    let mut phase_setting = [0i32; 5];
    let mut largest_signal = 0;
    for setting in settings {
        let mut signal = 0;
        let mut amplifiers = [
            interpreter.clone(),
            interpreter.clone(),
            interpreter.clone(),
            interpreter.clone(),
            interpreter.clone(),
        ];
        for index in 0..setting.len() {
            amplifiers[index].set_input(setting[index]);
        }

        while !amplifiers
            .iter()
            .all(|x| *x.get_state() == ExecutionState::Finished)
        {
            for amp in &mut amplifiers {
                amp.set_input(signal);
                amp.execute();
                signal = amp.get_output().expect("Signal value");
            }
        }
        if signal > largest_signal {
            largest_signal = signal;
            phase_setting = setting;
        }
    }

    println!(
        "Day 07.2: Largest output signal with feedback is {} for phase setting {:?}",
        largest_signal, phase_setting
    );
}
