#!/bin/bash

set -ue

flatpak install --user -y flathub org.flatpak.Builder
flatpak run org.flatpak.Builder --force-clean --user --install --install-deps-from=flathub --ccache --mirror-screenshots-url=https://dl.flathub.org/media/ --repo=repo builddir ./de.sebastianruziczka.looksyk.local.yml
flatpak build-bundle repo looksyk.flatpak de.sebastianruziczka.looksyk
