FROM debian:stable-20210816

RUN apt-get update && apt-get install -y ruby curl make gcc

# Install FPM for building the deb
RUN gem install fpm

# Install rustup for rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /src