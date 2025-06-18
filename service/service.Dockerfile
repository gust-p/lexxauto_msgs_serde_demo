# Multi-stage build for ROS2 service with r2r
FROM ros:humble AS ros2-base

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify Rust installation
RUN rustc --version && cargo --version

# Set up ROS2 environment
RUN echo "source /opt/ros/humble/setup.bash" >> ~/.bashrc
SHELL ["/bin/bash", "-c"]

# Build stage
FROM ros2-base AS builder

WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Set up ROS2 environment for build
RUN source /opt/ros/humble/setup.bash && \
    export AMENT_PREFIX_PATH=/opt/ros/humble && \
    export COLCON_PREFIX_PATH=/opt/ros/humble && \
    cargo build --release

# Remove dummy source
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
RUN source /opt/ros/humble/setup.bash && \
    export AMENT_PREFIX_PATH=/opt/ros/humble && \
    export COLCON_PREFIX_PATH=/opt/ros/humble && \
    cargo build --release

# Runtime stage
FROM ros:humble

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    ros-humble-rmw-cyclonedds-cpp \
    && rm -rf /var/lib/apt/lists/*


# Copy the binary from builder (note: binary name matches Cargo.toml package name)
COPY --from=builder /app/target/release/service /usr/local/bin/service


# Set environment variables
ENV RUST_LOG=info
ENV ROS_DOMAIN_ID=0
ENV RMW_IMPLEMENTATION=rmw_cyclonedds_cpp
ENV AMENT_PREFIX_PATH=/opt/ros/humble
ENV COLCON_PREFIX_PATH=/opt/ros/humble
ENV LD_LIBRARY_PATH=/opt/ros/humble/lib
ENV PATH=/opt/ros/humble/bin:$PATH
ENV PYTHONPATH=/opt/ros/humble/lib/python3.10/site-packages
ENV ROS_DISTRO=humble
ENV ROS_VERSION=2

# Create entrypoint script to source ROS2
COPY <<EOF /entrypoint.sh
#!/bin/bash
set -e
source /opt/ros/humble/setup.bash
exec "\$@"
EOF

RUN chmod +x /entrypoint.sh && \
    /entrypoint.sh


# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep service || exit 1

ENTRYPOINT ["/entrypoint.sh"]
CMD ["/usr/local/bin/service"]
