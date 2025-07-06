FROM sourcescan/cargo-near:0.15.0-rust-1.86.0

USER root
RUN apt update && apt install -y pkg-config libssl-dev
USER near
WORKDIR /workspaces/UnchainHer-Contracts/journal