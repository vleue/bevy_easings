#![deny(
    warnings,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

//! log diagnostics during run

use bevy::core::{Time, Timer};
use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::ecs::{IntoQuerySystem, Res, ResMut};
use bevy::prelude::stage;
use std::time::Duration;
use tracing::debug;

/// An App Plugin that prints diagnostics to the console
pub struct LogDiagnosticsPlugin {
    /// How often to log diagnostics, by default every seconds
    pub wait_duration: Duration,
    /// Filter for specific diagnostics
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

        app.add_system_to_stage(stage::POST_UPDATE, Self::log_diagnostics_system.system());
    }
}

impl LogDiagnosticsPlugin {
    /// Create a new `LogDiagnosticsPlugin` that will filter on the given diagnostics
    pub fn filtered(filter: Vec<DiagnosticId>) -> Self {
        LogDiagnosticsPlugin {
            filter: Some(filter),
            ..Default::default()
        }
    }

    fn log_diagnostic(diagnostic: &Diagnostic) {
        if let Some(value) = diagnostic.value() {
            if let Some(average) = diagnostic.average() {
                debug!(
                    "diagnostic: {:<80}: {:<10.4}  (avg {:.4})",
                    diagnostic.name, value, average
                );
            } else {
                debug!("diagnostic: {:<80}: {:<10.4}", diagnostic.name, value);
            }
        }
    }

    fn log_diagnostics_system(
        mut state: ResMut<LogDiagnosticsState>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
    ) {
        state.timer.tick(time.delta_seconds);
        if state.timer.finished {
            if let Some(ref filter) = state.filter {
                for diagnostic in filter.iter().map(|id| diagnostics.get(*id).unwrap()) {
                    Self::log_diagnostic(diagnostic);
                }
            } else {
                for diagnostic in diagnostics.iter() {
                    Self::log_diagnostic(diagnostic);
                }
            }
        }
    }
}
