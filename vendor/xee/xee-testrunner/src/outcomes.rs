use std::fmt::Write;

use crate::testcase::TestOutcome;

pub(crate) trait Outcomes {
    fn outcomes(&self) -> Vec<&TestCaseOutcome>;
    fn filtered(&self) -> usize;
    fn unsupported(&self) -> usize;

    fn supported(&self) -> usize {
        self.outcomes().len() + self.filtered()
    }

    fn total(&self) -> usize {
        self.outcomes().len() + self.filtered() + self.unsupported()
    }

    fn count<F>(&self, f: F) -> usize
    where
        F: Fn(&TestCaseOutcome) -> bool,
    {
        self.outcomes().iter().filter(|outcome| f(outcome)).count()
    }

    fn passed(&self) -> usize {
        self.count(|outcome| matches!(outcome.outcome, TestOutcome::Passed))
    }
    fn unexpected_error(&self) -> usize {
        self.count(|outcome| matches!(outcome.outcome, TestOutcome::UnexpectedError(..)))
    }
    fn failed(&self) -> usize {
        self.count(|outcome| matches!(outcome.outcome, TestOutcome::Failed(..)))
    }
    fn erroring(&self) -> usize {
        self.count(|outcome| {
            matches!(
                outcome.outcome,
                TestOutcome::RuntimeError(..)
                    | TestOutcome::CompilationError(..)
                    | TestOutcome::UnsupportedExpression(..)
                    | TestOutcome::Unsupported
                    | TestOutcome::EnvironmentError(..)
            )
        })
    }

    fn display(&self) -> String {
        let mut s = String::new();
        write!(s, "Total: {}", self.total()).unwrap();
        write!(s, " Supported: {}", self.supported()).unwrap();
        write!(s, " Passed: {}", self.passed()).unwrap();
        write!(s, " Failed: {}", self.failed()).unwrap();
        write!(s, " Error: {}", self.erroring()).unwrap();
        write!(s, " WrongE: {}", self.unexpected_error()).unwrap();
        write!(s, " Filtered: {}", self.filtered()).unwrap();
        write!(s, " Unsupported: {}", self.unsupported()).unwrap();
        s
    }
}

#[derive(Debug)]
pub struct TestCaseOutcome {
    pub(crate) test_case_name: String,
    pub(crate) outcome: TestOutcome,
}

impl TestCaseOutcome {
    pub(crate) fn new(test_case_name: &str, outcome: TestOutcome) -> Self {
        Self {
            test_case_name: test_case_name.to_string(),
            outcome,
        }
    }
}

#[derive(Debug)]
pub struct TestSetOutcomes {
    pub(crate) test_set_name: String,
    pub(crate) outcomes: Vec<TestCaseOutcome>,
    pub(crate) filtered: usize,
    pub(crate) unsupported: usize,
}

impl TestSetOutcomes {
    pub(crate) fn new(test_set_name: &str) -> Self {
        Self {
            test_set_name: test_set_name.to_string(),
            outcomes: Vec::new(),
            filtered: 0,
            unsupported: 0,
        }
    }

    pub(crate) fn add_outcome(&mut self, test_case_name: &str, outcome: TestOutcome) {
        self.outcomes
            .push(TestCaseOutcome::new(test_case_name, outcome));
    }

    pub(crate) fn add_filtered(&mut self) {
        self.filtered += 1;
    }

    pub(crate) fn add_unsupported(&mut self) {
        self.unsupported += 1;
    }

    pub(crate) fn failing_names(&self) -> Vec<String> {
        self.outcomes
            .iter()
            .filter(|outcome| !outcome.outcome.is_exactly_passed())
            .map(|outcome| outcome.test_case_name.clone())
            .collect()
    }

    pub(crate) fn has_failures(&self) -> bool {
        self.outcomes
            .iter()
            .any(|outcome| !outcome.outcome.is_exactly_passed())
    }
}

impl Outcomes for TestSetOutcomes {
    fn outcomes(&self) -> Vec<&TestCaseOutcome> {
        self.outcomes.iter().collect()
    }
    fn filtered(&self) -> usize {
        self.filtered
    }
    fn unsupported(&self) -> usize {
        self.unsupported
    }
}

pub struct CatalogOutcomes {
    pub(crate) outcomes: Vec<TestSetOutcomes>,
}

impl CatalogOutcomes {
    pub(crate) fn new() -> Self {
        Self {
            outcomes: Vec::new(),
        }
    }

    pub(crate) fn add_outcomes(&mut self, test_set_outcomes: TestSetOutcomes) {
        self.outcomes.push(test_set_outcomes);
    }

    pub(crate) fn has_failures(&self) -> bool {
        self.outcomes.iter().any(|test_set_outcome| {
            test_set_outcome
                .outcomes
                .iter()
                .any(|outcome| !outcome.outcome.is_exactly_passed())
        })
    }
}

impl Outcomes for CatalogOutcomes {
    fn outcomes(&self) -> Vec<&TestCaseOutcome> {
        self.outcomes
            .iter()
            .flat_map(|test_set_outcome| test_set_outcome.outcomes())
            .collect()
    }

    fn filtered(&self) -> usize {
        self.outcomes
            .iter()
            .map(|test_set_outcome| test_set_outcome.filtered())
            .sum()
    }

    fn unsupported(&self) -> usize {
        self.outcomes
            .iter()
            .map(|test_set_outcome| test_set_outcome.unsupported())
            .sum()
    }
}
