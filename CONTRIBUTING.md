# Contributing to rsxxd

Thank you for your interest in contributing to rsxxd! This document outlines the process for contributing to this project.

## Branch Structure

This repository follows a specific branch structure:

- **main**: The stable branch containing production-ready code. This branch is protected and only updated through reviewed pull requests from the `dev` branch.
- **dev**: The development branch where all features, fixes, and improvements are integrated before being promoted to `main`.

## Contribution Workflow

1. **Fork the repository** (if you're an external contributor) or create a new branch from `dev` (if you're a project member).

2. **Create a feature/fix branch** with a descriptive name:
   ```
   git checkout -b feature/your-feature-name
   ```
   or
   ```
   git checkout -b fix/issue-you-are-fixing
   ```

3. **Make your changes** on your branch.

4. **Commit your changes** with clear, descriptive commit messages:
   ```
   git commit -m "Add feature: description of changes"
   ```

5. **Push your branch** to GitHub:
   ```
   git push origin your-branch-name
   ```

6. **Create a Pull Request** targeting the `dev` branch, not `main`.

## Pull Request Guidelines

- All pull requests must target the `dev` branch.
- Include a clear description of the changes and their purpose.
- Reference any relevant issues using the GitHub issue number (e.g., "Fixes #123").
- Keep pull requests focused on a single feature or fix.
- Update documentation as needed.

## CI Testing

All pull requests must pass the Continuous Integration (CI) tests before they can be merged. These tests help ensure code quality and prevent regressions.

If your PR fails CI tests:
1. Check the CI logs to understand the issue.
2. Make the necessary fixes in your branch.
3. Push the changes to update your PR.
4. Wait for the CI to run again.

## Code Review

All contributions will be reviewed before merging. Reviewers may suggest changes or improvements. Please be responsive to feedback and update your PR accordingly.

Thank you for contributing to rsxxd!