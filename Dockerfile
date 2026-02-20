FROM node:20

ARG CLAUDE_CODE_VERSION=latest

# Install basic development tools and iptables/ipset
RUN apt-get update && apt-get install -y --no-install-recommends \
  less \
  git \
  procps \
  sudo \
  fzf \
  zsh \
  man-db \
  unzip \
  gnupg2 \
  gh \
  dnsutils \
  aggregate \
  jq \
  vim \
  libasound2-dev \
  pkg-config \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

# Grant node user passwordless sudo
RUN echo "node ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/node

# Ensure default node user has access to /usr/local/share
RUN mkdir -p /usr/local/share/npm-global && \
  chown -R node:node /usr/local/share

# Create workspace and config directories and set permissions
RUN mkdir -p /home/node/.claude && \
  chown -R node:node /home/node/.claude

WORKDIR /home/node

# Install Rust toolchain
USER node
ENV RUSTUP_HOME=/home/node/.rustup
ENV CARGO_HOME=/home/node/.cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/node/.cargo/bin:$PATH"

# Install global packages
ENV NPM_CONFIG_PREFIX=/usr/local/share/npm-global
ENV PATH=$PATH:/usr/local/share/npm-global/bin

ENV SHELL=/bin/bash

# Set the default editor and visual
ENV EDITOR=vi
ENV VISUAL=vi

# Install Claude
RUN npm install -g @anthropic-ai/claude-code@${CLAUDE_CODE_VERSION}

