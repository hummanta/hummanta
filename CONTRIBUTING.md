# Contribution Guide

Welcome to the project! We greatly appreciate your contributions. Before submitting your contribution, please take a moment to read the following guidelines.

## How to Contribute

### Reporting Issues

If you find a bug or have a feature request, please first check the [issue tracker](https://github.com/hummanta/hummanta/issues) to see if the issue already exists. If it doesn't, please create a new issue and provide as much detail as possible, including:

- A description of the issue
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Relevant logs or screenshots (if applicable)

### Submitting Pull Requests

1. **Fork the Repository**: First, fork this project to your GitHub account.

2. **Clone the Repository**: Clone the forked repository to your local machine.

   ```bash
   git clone https://github.com/hummanta/hummanta.git
   cd hummanta
   ```

3. **Create a Branch**: Create a new branch for your changes.

   ```bash
   git checkout -b your-branch-name
   ```

4. **Make Changes**: Make your changes locally.

5. **Run Tests**: Ensure all tests pass before submitting your changes.

   ```bash
   just test
   ```

6. **Commit Changes**: Commit your changes with a clear commit message.

   ```bash
   git add .
   git commit -m "Describe your changes"
   ```

7. **Push Changes**: Push your changes to GitHub.

   ```bash
   git push origin your-branch-name
   ```

8. **Create a Pull Request**: Create a Pull Request on GitHub and describe your changes. Make sure your PR includes the following:

   - The purpose of the changes
   - Related issue number (if applicable)
   - Any other relevant context

### Code Style

Please follow the project's code style guidelines. Here are some general Rust code style recommendations:

- Use `just fmt` to format your code.
- Use `just clippy` for code linting.

### Testing

Ensure your changes include appropriate tests and that all tests pass. Run the following command to execute tests:

```bash
just test
```

### Documentation

If your changes involve public APIs or features, please make sure to update the relevant documentation.

## License

By submitting contributions, you agree that your contributions will be licensed under the project's [LICENSE](LICENSE).

---

Thank you for contributing! 🎉
