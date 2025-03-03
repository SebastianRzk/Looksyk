# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: Your Name <youremail@domain.com>
pkgname=looksyk-desktop-git
_pkgnameshort=looksyk
pkgver=1.4.2
pkgrel=1
pkgdesc="A markdown centric, fast and local personal knowledge platform"
arch=("x86_64")
url="https://sebastianrzk.github.io/Looksyk"
license=('AGPL')
groups=()
depends=(electron)
makedepends=(git nvm npm cargo)
checkdepends=(cargo)
provides=(looksyk looksyk-backend)
changelog=
source=("git+https://github.com/sebastianrzk/looksyk#tag=v$pkgver")
sha256sums=('SKIP')

prepare() {
	cd "$_pkgnameshort"
	#nvm install 23.5
	cd frontend/looksyk
	npm install
	cd ../..
	cd application-wrapper/Looksyk
	npm install
	cd ../..
}

build() {
	cd "$_pkgnameshort"
	cd backend
	CFLAGS+=' -ffat-lto-objects' cargo build --release
	cd ..
	cd frontend/looksyk
	npm run build --configuration=production
	cd ../..
	cd application-wrapper/Looksyk
	npm run package
	cd ..
}

check() {
	cd "$_pkgnameshort"
	cd backend
	CFLAGS+=' -ffat-lto-objects' cargo test
}

package() {
	cd "$_pkgnameshort"
	mkdir -p "${pkgdir}/usr/share/${_pkgnameshort}"
	install -d "${pkgdir}/usr/share/" "${pkgdir}/usr/bin/"
	install -D -m644 "LICENSE" "${pkgdir}/usr/share/licenses/${_pkgnameshort}/LICENSE"
	install -D -m644 "application-wrapper/Looksyk/out/looksyk-linux-x64/resources/app.asar" "${pkgdir}/usr/share/${_pkgnameshort}/app.asar"
	install -D -m755 "backend/target/release/looksyk" "${pkgdir}/usr/share/${_pkgnameshort}/looksyk-backend"
	install -D -m755 "application-wrapper/looksyk" "${pkgdir}/usr/share/${_pkgnameshort}/looksyk"
	cp -r "frontend/looksyk/dist/looksyk/browser/" "${pkgdir}/usr/share/${_pkgnameshort}/static/"
	
	ln -s "/usr/share/${_pkgnameshort}/looksyk-backend" "${pkgdir}/usr/bin/looksyk-backend"
	ln -s "/usr/share/${_pkgnameshort}/looksyk" "${pkgdir}/usr/bin/looksyk"
}
