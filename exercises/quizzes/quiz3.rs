// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

use std::fmt;

struct Grade {
    float: Option<f32>,
    alphabetic: Option<String>,
}

impl Grade {
    fn from_float(grade: f32) -> Self {
        Self {
            float: Some(grade),
            alphabetic: None,
        }
    }

    fn from_alphabetic(grade: &str) -> Self {
        Self {
            float: None,
            alphabetic: Some(grade.to_string()),
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.float, &self.alphabetic) {
            (Some(float), None) => write!(f, "{:.1}", float),
            (None, Some(alphabetic)) => write!(f, "{}", alphabetic),
            _ => panic!("Grade must be either numeric or alphabetic"),
        }
    }
}

struct ReportCard {
    grade: Grade,
    student_name: String,
    student_age: u8,
}

impl ReportCard {
    fn new(grade: Grade, student_name: String, student_age: u8) -> Self {
        Self {
            grade,
            student_name,
            student_age,
        }
    }

    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::from_float(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: Grade::from_alphabetic("A+"),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
