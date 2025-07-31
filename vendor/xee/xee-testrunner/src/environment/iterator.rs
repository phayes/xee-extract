use crate::error::{Error, Result};

use super::{Environment, SharedEnvironments, TestCaseEnvironment};

pub(crate) struct EnvironmentIterator<'a, E: Environment> {
    pub(crate) inherited_shared_environments: Vec<&'a SharedEnvironments<E>>,
    pub(crate) environments: &'a [TestCaseEnvironment<E>],
    pub(crate) index: usize,
}

impl<'a, E: Environment> EnvironmentIterator<'a, E> {
    pub(crate) fn new(
        inherited_shared_environments: Vec<&'a SharedEnvironments<E>>,
        test_case_environments: &'a [TestCaseEnvironment<E>],
    ) -> Self {
        Self {
            inherited_shared_environments,
            environments: test_case_environments,
            index: 0,
        }
    }
}

impl<'a, E: Environment> Iterator for EnvironmentIterator<'a, E> {
    type Item = Result<&'a E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.environments.len() {
            return None;
        }
        let environment = &self.environments[self.index];
        self.index += 1;
        match environment {
            TestCaseEnvironment::Local(local_environment_spec) => Some(Ok(local_environment_spec)),
            TestCaseEnvironment::Ref(environment_ref) => {
                for shared_environments in &self.inherited_shared_environments {
                    let environment_spec = shared_environments.get(environment_ref);
                    if let Some(environment_spec) = environment_spec {
                        return Some(Ok(environment_spec));
                    }
                }
                Some(Err(Error::UnknownEnvironmentReference(
                    environment_ref.clone(),
                )))
            }
        }
    }
}
