pkgname=teapot
pkgdesc='The teapot go spin spin'
pkgver=0.1.1
pkgrel=1
makedepends=('rust' 'cargo')
arch=('any')
source=("git+https://github.com/Vonercent/teapot")
sha256sums=('SKIP')
license=('MIT')

# Generated in accordance to https://wiki.archlinux.org/title/Rust_package_guidelines.
# Might require further modification depending on the package involved.
prepare() {
  cd $pkgname
  cargo fetch --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd $pkgname
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

package() {
  cd $pkgname
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
  install -Dm644 "$pkgname.desktop" "${pkgdir}/usr/share/applications/$pkgname.desktop"
}
