use bevy::core::{Time, Timer};
use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::ecs::{IntoQuerySystem, Res, ResMut};
use bevy::prelude::stage;
use std::time::Duration;
use tracing::debug;

/// An App Plugin that prints diagnostics to the console
pub struct LogDiagnosticsPlugin {
    pub wait_duration: Duration,
    pub filter: Option<Vec<DiagnosticId>>,
}

/// State used by the [LogDiagnosticsPlugin]
pub struct LogDiagnosticsState {
    timer: Timer,
    filter: Option<Vec<DiagnosticId>>,
}

impl Default for LogDiagnosticsPlugin {
    fn default() -> Self {
        LogDiagnosticsPlugin {
            wait_duration: Duration::from_secs(1),
            filter: None,
        }
    }
}

impl bevy::prelude::Plugin for LogDiagnosticsPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_resource(LogDiagnosticsState {
            timer: Timer::new(self.wait_duration, true),
            filter: self.filter.clone(),
        });

        app.add_system_to_stage(stage::POST_UPDATE, Self::print_diagnostics_system.system());
    }
}

impl LogDiagnosticsPlugin {
    pub fn filtered(filter: Vec<DiagnosticId>) -> Self {
        LogDiagnosticsPlugin {
            filter: Some(filter),
            ..Default::default()
        }
    }

    fn print_diagnostic(diagnostic: &Diagnostic) {
        if let Some(value) = diagnostic.value() {
            if let Some(average) = diagnostic.average() {
                debug!(
                    "diagnostic: {:<25}: {:<10.4}  (avg {:.4})",
                    diagnostic.name, value, average
                );
            } else {
                debug!("diagnostic: {:<25}: {:<10.4}", diagnostic.name, value);
            }
        }
    }

    pub fn print_diagnostics_system(
        mut state: ResMut<LogDiagnosticsState>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
    ) {
        state.timer.tick(time.delta_seconds);
        if state.timer.finished {
            if let Some(ref filter) = state.filter {
                for diagnostic in filter.iter().map(|id| diagnostics.get(*id).unwrap()) {
                    Self::print_diagnostic(diagnostic);
                }
            } else {
                for diagnostic in diagnostics.iter() {
                    Self::print_diagnostic(diagnostic);
                }
            }
        }
    }
}
