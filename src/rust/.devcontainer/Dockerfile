FROM rust:1

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Configure apt and install packages
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils 2>&1 \
    #
    # Verify git, needed tools installed
    && apt-get -y install git procps lsb-release \
    #
    # Install other dependencies
    && apt-get install -y lldb-3.9 \
    #
    # Install clang
    # From https://gist.github.com/twlz0ne/9faf00346a2acf10044c54f9ba0b9805#file-dockerfile
    && apt-get update && apt-get install -y gnupg wget software-properties-common \
    && wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - \
    && apt-add-repository "deb http://apt.llvm.org/stretch/ llvm-toolchain-stretch-6.0 main" \
    && apt-get update && apt-get install -y clang-6.0 \
    #
    # Install CMake
    && wget https://cmake.org/files/v3.8/cmake-3.8.2-Linux-x86_64.sh \
    && mkdir /opt/cmake \
    && sh cmake-3.8.2-Linux-x86_64.sh --prefix=/opt/cmake --skip-license \
    && ln -s /opt/cmake/bin/cmake /usr/local/bin/cmake \
    #
    # Install Rust components
    && rustup update \
    && rustup component add rls rust-analysis rust-src \
    #
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=dialog
