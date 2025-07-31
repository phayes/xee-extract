# CI

How does continuous integration work? (CI)

This document applies to this repository, but also to the other xee-related
repositories:

- xee
- regexml
- xee-format
- xee-php
- spellout
- xoz

## release-plz

We're using the [release-plz](https://release-plz.dev/) system for releasing to
crates.io.

This creates a release PR automatically whenever there is a change to the code
on main. This contains updated changelogs. These changelogs can be edited on
the PR to clean them up and make them more readable, as they're based on git
commits.

After it the PR is merged, release-plz automaticaly releases the Rust crates to
crates.io.

## crates.io token

In order to publish to crates.io, an access token is needed. This was created
by Martijn Faassen (`faassen` on github) on his crates.io account. The token
cannot touch all crates.io published by Martijn; instead it has been restricted
so it can only touch crates that start with `xee`, `regexml`, and `spellout`.
If ever any release needs to be made to a crate with a name that isn't prefixed
'xee' or 'regexml', then this access needs to be extended (see also "future
improvements" below).

This token is named CRATES_IO_XEE. It's set up separately for each xee-related
repository at the time being. 

## ownership on crates.io

All xee-related crates.io packages have ownership access added for xee-team in
the Paligo github organization (`github:paligo:xee-team`). This means that
anyone in the Paligo xee-team shares ownership with the crates.io packages and
can manage them through crates.io should this be necessary.

## future improvements

Should Paligo want to take over management of the crates.io token that
requires:

- Use a Paligo controlled github account that's in the Paligo `xee-team`. This
  gives it owner access to the Xee-related crates. This could be some other
  github user's account, or a Paligo specific machine account. When creating a
  fine-grained PAT you need to create the Paligo organization. [More
  info](https://release-plz.dev/docs/github/token)

- Create a Paligo controlled `crates.io` account associated with that github
  account.

- On `crates.io`, In "Account Settings" -> "API Tokens" create a new token with
  scopes `publish-new` and `publish-update`. Unless this user plans to publish
  non-xee related crates, it doesn't need crate restrictions.
  
- Remove the `CRATES_IO_XEE` tokens for the specific Xee repositories on
  github, and add a single `CRATES_IO_XEE` token to the Paligo organization.
  (in "Secrets and Variables" -> "Actions")



