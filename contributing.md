# Contributing

Thank you for considering contributing to `choose`!

To save your time and mine, I will attempt to maintain brevity in this
document, adding more details where there are common questions or
misunderstandings.

## Where To Start

If you have found a bug or would like to request a feature, [open an
issue](https://github.com/theryangeary/choose/issues/new).

It is best if you get confirmation of your bug or approval for a feature
request before taking the time to write the code.

## Fork && Create Branch

If you have confirmation/approval and would like to try your hand at making the
change, [fork `choose`](https://help.github.com/articles/fork-a-repo) and create
a branch with a descriptive name.

Branch off of `develop`. Bug fix branches should be named
`hotfix/<descriptive-name>` and feature branches should be named
`feature/<descriptive-name>`. **Any hotfix or feature branch should only address
one issue/feature**.

```sh
git checkout -b <branch-name> develop
```

## Check The Test Suite

Before making any changes, make sure that both the unit tests and the end-to-end
tests all work.

```sh
cargo test
test/e2e_test.sh
```

If you are planning on making changes that may affect performance, consider
using the benchmark script `test/bench.sh` as well.

## Implement Your Fix/Feature

### Write tests

Tests are important.

If you are fixing a bug, add tests that identify that bug and any permutations
of it you can find, so we can ensure it doesn't come back.

If you are creating a feature, add tests that will comprehensively ensure the
feature works as expected, in conjunction with all other features, switches,
options, etc.

### Write code

It should be correct. It should be fast. It should be idiomatic. Ask for help if
you need it, don't be shy.

### Write documentation (if needed)

If your feature adds a new command line switch or option, add that info to the
readme or any other relevant locations.

## Make a Pull Request

Once you've finished your changes, make sure that your develop branch is up to
date.

```sh
git remote add upstream git@github.com:theryangeary/choose.git
git checkout develop
git pull upstream develop
```

Check that your code is all formatted correctly. If not, commit any changes.

```sh
git checkout <your-branch>
cargo fmt
git status
```

Rebase and squash your branch on develop. This will prompt you with a list of
your commits. Change all but the first commit to "squash". Write a nice
changelog message in the resulting commit.

```sh
git rebase -i develop
```

Push to your fork.

```sh
git push --set-upstream origin <your-branch>
```

Go to GitHub and [make a Pull
Request](https://help.github.com/articles/creating-a-pull-request)! Make sure
that your Pull Request is against `develop` and not `master`!

## Keep Your Pull Request Updated

After making your Pull Request, you may be asked to make some changes. After
completing and commiting your changes, you will need to rebase and resquash your
commits. Each Pull Request will effectively be a single commit added to the
`develop` branch.

After changing and committing, push like this:

```sh
git push --force-with-lease <your-branch>
```
