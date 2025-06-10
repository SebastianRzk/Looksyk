---
layout: default
---

# Development Build

## Table of Contents

### This page

- [Build backend and frontend](#build-backend-and-frontend)
- [Build electron app](#build-electron-app)
- [Contribution Guidelines](#contribution-guidelines)

### Further Reading

- [Overview](index.md)
- [idea and technical concept](idea_and_technical_concept.md)
- [Install + Run](installation.md)
- [Migrate Your Existing Logseq Graph](migration_from_logseq.md)
- [Configuration and Usage](usage.md)
- [Development Guide and Contribution Guidelines](development_and_contribution.md)
- [Changelog](changelog.md)

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
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request into the `dev` branch.


## Troubleshooting

- see FAQ [Install + Run](installation.md)
