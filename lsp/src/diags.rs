use diagnostics::diagnostic::Level;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Position, Range, Url,
};

use crate::main;

pub fn to_tower_diag(diagnostic: diagnostics::diagnostic::Diagnostic) -> Diagnostic {
    let severity = match diagnostic.level {
        Level::Error => DiagnosticSeverity::ERROR,
        Level::Warning => DiagnosticSeverity::WARNING,
        Level::Note => DiagnosticSeverity::INFORMATION,
    };

    let mut related_info = vec![];

    for span in diagnostic.spans {
        related_info.push(DiagnosticRelatedInformation {
            location: Location {
                uri: Url::parse(&format!("file:///{}", span.start.file_path)).unwrap(),
                range: Range {
                    start: Position::new(span.start.line as u32, span.start.col as u32),
                    end: Position::new(span.start.line as u32, span.start.end_col as u32),
                },
            },
            message: span.label.unwrap().into(),
        });
    }

    let main_range = Range {
        start: Position::new(
            diagnostic.primary_span.start.line as u32,
            diagnostic.primary_span.start.col as u32,
        ),
        end: Position::new(
            diagnostic.primary_span.start.line as u32,
            diagnostic.primary_span.start.end_col as u32,
        ),
    };

    if diagnostic.primary_span.label.is_some() {
        related_info.push(DiagnosticRelatedInformation {
            location: Location {
                uri: Url::parse(&format!(
                    "file:///{}",
                    diagnostic.primary_span.start.file_path
                ))
                .unwrap(),
                range: main_range.clone(),
            },
            message: diagnostic.primary_span.label.unwrap(),
        })
    }

    Diagnostic {
        range: main_range,
        message: diagnostic.message,
        severity: Some(severity),
        related_information: Some(related_info),

        ..Default::default()
    }
}
