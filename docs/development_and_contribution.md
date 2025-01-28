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

1. Clone the repository
2. Install frontend dependencies and run `cd frontend/looksyk`, `npm install`, `ng serve`
3. Install backend dependencies and run `cd backend` and `cargo run`
4. The application is now available at `http://localhost:4200` (frontend) and `http://localhost:8989` (backend)

## Build electron app

1. Change into the application-wrapper directory `cd application-wrapper`
2. Install electron dependencies `npm install`
3. Build the electron app `npm run package`
4. The application is now in the `out` folder e.g. `out/looksyk-linux-x64/looksyk`


## Contribution Guidelines

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.