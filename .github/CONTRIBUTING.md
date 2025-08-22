# Contributing

## Request for Changes / Pull Requests

You first need to create a fork of the [rhyolite](https://github.com/RedddFoxxyy/rhyolite) repository to commit your
changes to it. Methods to fork a repository can be found in
the [GitHub Documentation](https://docs.github.com/en/get-started/quickstart/fork-a-repo).

Then add your fork as a local project:

```sh
git clone https://github.com/YOUR-USERNAME/rhyolite
```

> [Which remote URL should be used?](https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories)

Then, go to your local folder:

```sh
cd rhyolite
```

Add git remote controls:

```sh
git remote add fork https://github.com/YOUR-USERNAME/rhyolite
git remote add upstream https://github.com/RedddFoxxyy/rhyolite
```

You can now verify that you have your two git remotes:

```sh
git remote -v
```

## Receive Remote Updates

To stay up to date with the central repository:

```sh
git pull upstream master
```

## Choose a Base Branch

Before starting development, you need to know which branch to base your modifications/additions on. When in doubt, use
`master`.

### Branch Naming Convention

When creating a new branch, use the following naming pattern based on the type of change:

- **Features**: `feat/description` (e.g., `feat/file-saving`, `feat/user-authentication`)
- **Bug fixes**: `fix/description` (e.g., `fix/memory-leak`, `fix/login-error`)
- **Documentation**: `docs/description` (e.g., `docs/api-guide`, `docs/readme-update`)
- **Refactoring**: `refactor/description` (e.g., `refactor/database-layer`)
- **Performance**: `perf/description` (e.g., `perf/query-optimization`)
- **Tests**: `test/description` (e.g., `test/user-service`)
- **Chores**: `chore/description` (e.g., `chore/dependency-update`)

```sh
# Switch to the desired branch
git switch master
# Pull down any upstream changes
git pull upstream master
# Create a new branch to work on (replace with appropriate prefix)
git switch --create feat/your-feature-name
```

## Submitting Your Pull Request

1. **Make your changes** and commit them to your branch
2. **Push your branch** to your fork:
   ```sh
   git push -u fork feat/your-feature-name
   ```
3. **Create a draft pull request** on the [main rhyolite repository](https://github.com/RedddFoxxyy/rhyolite) targeting
   the `master` branch
4. **Mark as ready for review** when your changes are complete
5. **Request a review** from a maintainer
6. **Wait for approval** - a maintainer will review your changes and squash merge the PR when approved

### Pull Request Quality Guidelines

**Do NOT submit low-quality pull requests such as:**

- Simple name fixes or variable renaming
- Grammar corrections in comments or documentation
- Code style changes or formatting suggestions
- Suggesting new coding standards or conventions

**Use repository issues instead** for:

- Reporting typos or grammar issues
- Suggesting code style improvements
- Proposing new coding conventions
- Small documentation fixes

**Good pull requests should:**

- Add meaningful functionality or fix substantial bugs
- Include tests for new features
- Address existing issues from the issue tracker
- Provide clear value to the project

## Code Style Guidelines

Please follow these code style guidelines when contributing:

### Formatting

- **Indentation**: Use tab indents with tab width set to 4 spaces
- **Line Length**: Maximum line width of 140 characters
- **Linting**: Follow Clippy linting rules - ensure your code passes `cargo clippy` without warnings

### Running Code Quality Checks

Before submitting your pull request, make sure to run:

```sh
# Format your code
cargo fmt

# Check for linting issues
cargo clippy

# Run tests
cargo test
```

These guidelines help maintain consistent code quality and readability across the project.