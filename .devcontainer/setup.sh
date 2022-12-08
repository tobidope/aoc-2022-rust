## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  vim \
  build-essential \
  openssl \
  libssl-dev

## Install rustup and common components
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 
source "$HOME/.cargo/env"
cargo install cargo-expand
cargo install cargo-edit
cargo install cargo-cache