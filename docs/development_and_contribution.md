---
layout: base.njk
title: Development and Contribution
---

# Development and Contribution

Thank you for your interest in this project! It's a private open-source hobby project that I maintain in my free time. Therefore, there are some simple but important rules for contributions:

## Project Goal
This project is primarily intended to remain functional, stable, and easy to understand. It is available for free, but maintenance is provided without commercial support and for a limited time.

## Contributions are welcome!
Feel free to contribute, e.g. E.g., with:
- Functional enhancements
- Bug fixes
- Documentation improvements
- Suggestions that provide clear added value

## What I don't want
Please refrain from:
- Discussions about purely stylistic issues (e.g., branch names, commit formats) if they don't solve functional problems
- Criticism without concrete solutions or practical benefits
- PRs or issues that restructure existing functionality without a clear benefit

## Submitting suggestions
If you have ideas or requests:
1. Please describe specifically what you are suggesting.
2. Why is it helpful?
3. If possible, offer a solution or PR right away.


## Writing PRs
I value well-structured code. All code styles and best practices are automatically reviewed by various GitHub Actions.

Please ensure that your changes do not violate the guidelines beforehand. To do so, run the file `bash run-ci-manual.sh` and check that the text "all done" is displayed and that no changes have been made automatically in the Git repository.


I reserve the right to close issues or pull requests that don't fit the project framework or that require too much maintenance without providing any benefit.

Thank you for your understanding and enjoy the project!


--- 

# Development Setup

## Build backend and frontend

Build and start the backend in development mode. The frontend will reload automatically when changes are made, the
backend will need to be restarted manually.

1. Clone the repository.
2. Navigate to the frontend directory: `cd frontend/looksyk`
3. Install frontend dependencies: `npm install`
4. Start the frontend server: `npm run ng serve --proxy-config=proxy.config.json`
5. Navigate to the backend directory: `cd backend`
6. Install backend dependencies: `cargo run`
7. The frontend will be available at `http://localhost:4200`
8. The backend will be available at `http://localhost:11000`

## Run backend tests

1. Navigate to the backend directory: `cd backend`
2. Run the tests: `cargo test`

If there are changes at the high-efficiency parts of the codebase (e.g. the parser), it's recommended to run the
benchmark tests as well:

1. Navigate to the backend directory: `cd backend`
2. Run the benchmark tests: `cargo bench`
3. Make your changes
4. Run the benchmark tests again to compare the results

## Build electron app

1. Change into the application-wrapper directory `cd application-wrapper/Looksyk/`
2. Install electron dependencies `npm install`
3. Build the electron app `npm run package`
4. The application is now in the `out` folder e.g. `out/looksyk-linux-x64/looksyk`

## Contribution Guidelines

Contributions are welcome! Here are the steps to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Check if your changes pass the tests and checks. Run `bash run-ci-manual.sh` to run all tests and checks. If there
   are
   any issues, please fix them before proceeding.
5. Commit your changes (`git commit -am 'Add new feature'`).
6. Push to the branch (`git push origin feature-branch`).
7. Create a new Pull Request into the `main` branch. Here, all changes are collected, manually tested by me, and
   combined into a single version. To release a version, the main branch is merged into the stable branch.~~~~

## Where should I target my pull request?

Pull requests should be targeted to the `main` branch.

The `stable` branch is reserved for the latest release.

## Why is the `stable` branch not the default branch?

The `stable` branch is not the default branch because it is reserved for the latest release. The `main` branch is where
active development takes place, and it is the branch that contributors should target for their pull requests.

GitHub will automatically show the default branch when you visit the repository. To avoid unstable or incomplete changes
being displayed by default, the default branch is `stable`. In addition, the GitHub docs website is also generated from
the default branch.

## Troubleshooting

- see FAQ [Install + Run]({{config.pathPrefix}}installation/)
