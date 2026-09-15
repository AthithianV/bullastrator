# Contributing to Asyncian

Thank you for your interest in contributing to **Asyncian**

Asyncian is an open-source desktop application for managing and inspecting Redis & BullMQ queues. Contributions of all kinds are welcome — bug fixes, features, documentation, UI improvements, performance improvements, and ideas.

## Getting Started

### Prerequisites

Before contributing, make sure you have:

- Node.js
- Rust
- Redis
- Tauri prerequisites for your operating system
- Git

### Clone the repository

```bash
git clone https://github.com/athithianv/asyncian.git
cd asyncian
```

### Install dependencies

```bash
npm install
```

### Start the development application

```bash
npm run tauri dev
```

Make sure Redis is running locally or connect to another Redis instance from Asyncian.

---

## Development Workflow

### 1. Create a branch

Create a branch from `main`.

```bash
git checkout -b feature/my-feature
```

Use descriptive branch names:

```text
feature/queue-search
feature/job-templates
fix/job-retry
fix/redis-connection
docs/getting-started
refactor/job-service
```

### 2. Make your changes

Keep changes focused and avoid mixing unrelated changes in the same pull request.

### 3. Test your changes

Before opening a pull request, make sure:

- The application builds successfully
- Existing functionality still works
- New functionality has been tested
- No credentials or secrets are included
- Formatting and linting pass

### 4. Commit your changes

Write clear commit messages.

Examples:

```text
feat: add queue search
fix: handle failed job retry errors
refactor: simplify redis connection service
docs: improve installation instructions
```

### 5. Open a Pull Request

Push your branch:

```bash
git push origin feature/my-feature
```

Then open a Pull Request against `main`.

---

## Pull Requests

A good Pull Request should:

- Clearly explain what changed
- Explain why the change was necessary
- Include screenshots for UI changes
- Include relevant testing information
- Keep the scope focused
- Avoid unrelated formatting changes

For UI changes, screenshots or short recordings are highly encouraged.

---

## Reporting Bugs

Before opening a bug report:

1. Search existing issues.
2. Make sure you're using the latest version.
3. Confirm that the issue is reproducible.

Include:

- Operating system
- Asyncian version
- Redis version
- Steps to reproduce
- Expected behavior
- Actual behavior
- Relevant logs
- Screenshots or recordings when useful

**Never include Redis passwords, API keys, tokens, or other secrets in an issue.**

---

## Suggesting Features

Feature requests are welcome.

Please explain:

- What problem the feature solves
- How you expect it to work
- Why it would be useful
- Any examples or references

A proposed implementation is welcome but not required.

---

## Code Style

Follow the existing conventions in the project.

Prefer:

- Small, focused functions
- Clear names
- Simple abstractions
- Explicit error handling
- Reusable components
- Minimal unnecessary dependencies

Avoid introducing abstractions unless they provide a clear benefit.

---

## Security

Never commit:

- Passwords
- API keys
- Redis credentials
- Private keys
- `.env` files containing secrets
- Production connection information

If you discover a security vulnerability, please follow the instructions in [`SECURITY.md`](SECURITY.md) instead of opening a public issue.

---

## License

By contributing to Asyncian, you agree that your contributions will be licensed under the same license as the project.
