FROM debian:stable-20210816
MAINTAINER Adam Schwalm <adamschwalm@gmail.com>

RUN apt-get update && apt-get install -y ruby curl make gcc

# Install FPM for building the deb
RUN gem install fpm

# Install rustup for rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:$PATH"
ENV RUSTUP_HOME /root/.rustup
ENV CARGO_HOME /root/.cargo

RUN rustup default stable

WORKDIR /src