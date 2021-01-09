FROM ubuntu:20.10

RUN apt-get update && \
    apt-get install -y git cmake pkg-config libboost-all-dev libtbb-dev \
    libcurl4-gnutls-dev libpng-dev libgl1-mesa-dev libnlopt-cxx-dev \
    libopenvdb-dev g++ libcgal-dev libdbus-1-dev curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /

RUN git clone https://github.com/USCiLab/cereal.git
WORKDIR /cereal
RUN git checkout develop
RUN mkdir -p build
WORKDIR /cereal/build

RUN cmake .. -DJUST_INSTALL_CEREAL=ON
RUN make
RUN make install

WORKDIR /
RUN git clone https://github.com/prusa3d/PrusaSlicer
WORKDIR /PrusaSlicer

RUN git checkout tags/version_2.3.0-rc2

RUN mkdir /PrusaSlicer/build
WORKDIR /PrusaSlicer/build
RUN cmake .. -DSLIC3R_GUI=0

RUN apt-get install -y libclang-dev clang && \
    rm -rf /var/lib/apt/lists/*

VOLUME "/data"
WORKDIR /data

CMD ["cargo", "build"]