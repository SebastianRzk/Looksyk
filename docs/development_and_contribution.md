---
layout: base.njk
title: Development and Contribution
---

# Development Build

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
4. Check if your changes pass the tests and checks. Run `bash run-ci-manual.sh` to run all tests and checks. If there are
   any issues, please fix them before proceeding.
5. Commit your changes (`git commit -am 'Add new feature'`).
6. Push to the branch (`git push origin feature-branch`).
7. Create a new Pull Request into the `main` branch.

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
