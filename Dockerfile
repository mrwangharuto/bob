# Use Ubuntu as the base image
FROM --platform=$TARGETPLATFORM ubuntu:22.04

# Prevent timezone prompt during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install essential build tools and Rust dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust binaries to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify installation
RUN rustc --version && \
    cargo --version

# Set the working directory
WORKDIR /app

# Default command (can be overridden)
CMD ["bash"]