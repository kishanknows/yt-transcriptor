# building stage
FROM rust:latest AS builder

# Install dependencies for bindgen
RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    llvm-dev \
    pkg-config \
    libssl-dev \
    cmake \
    g++ \
    build-essential \
    libc6-dev \
    libstdc++-12-dev \
    libatomic1 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# set LIBCLANG_PATH so bindgen can find libclang
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib

RUN rustup component add rustfmt

WORKDIR /app
COPY . .

# Download GGML model into /app/llms
RUN mkdir -p /app/llms && \
    curl -L -o /app/llms/ggml-tiny.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin

# Pre-fetch dependencies so we can patch before build
RUN cargo fetch

# Patch whisper.cpp's CMakeLists.txt inside whisper-rs-sys
RUN find /usr/local/cargo/registry/src -type f -path "*/whisper.cpp/CMakeLists.txt" \
    -exec sed -i '1i set(GGML_NATIVE OFF)\nset(GGML_CPU_ARM_ARCH native)' {} \;

RUN cargo build --release

# running stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libssl3 \
    libstdc++6 \
    libatomic1 \
    ffmpeg \
    python3 \
    python3-pip \
    && pip3 install --no-cache-dir --break-system-packages yt-dlp \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/llms /app/llms

# Railway will set $PORT automatically
ENV PORT=8080
EXPOSE 8080

CMD [ "./backend" ]
