#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalibrationStep {
    Warmup,
    Brightness,
    Contrast,
    Gamma,
    WhitePoint,
    Verify,
    Complete,
}

impl CalibrationStep {
    pub fn all() -> &'static [CalibrationStep] {
        &[
            CalibrationStep::Warmup,
            CalibrationStep::Brightness,
            CalibrationStep::Contrast,
            CalibrationStep::Gamma,
            CalibrationStep::WhitePoint,
            CalibrationStep::Verify,
            CalibrationStep::Complete,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct CalibrationState {
    active: bool,
    current_step: usize,
}

impl Default for CalibrationState {
    fn default() -> Self {
        Self {
            active: false,
            current_step: 0,
        }
    }
}

impl CalibrationState {
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn start(&mut self) {
        self.active = true;
        self.current_step = 0;
    }

    pub fn cancel(&mut self) {
        self.active = false;
        self.current_step = 0;
    }

    pub fn finish(&mut self) {
        self.active = false;
        self.current_step = 0;
    }

    pub fn current_step(&self) -> CalibrationStep {
        CalibrationStep::all()
            .get(self.current_step)
            .copied()
            .unwrap_or(CalibrationStep::Complete)
    }

    pub fn step_number(&self) -> usize {
        self.current_step + 1
    }

    pub fn total_steps(&self) -> usize {
        CalibrationStep::all().len()
    }

    pub fn next_step(&mut self) {
        if self.current_step < CalibrationStep::all().len() - 1 {
            self.current_step += 1;
        }
    }

    pub fn previous_step(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
        }
    }
}
