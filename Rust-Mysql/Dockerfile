################
# Build system #
################
FROM rust as builder
# use the global variable
ARG RUST_APP

# create project, copy dependencies and build with default src
RUN USER=root cargo new ${RUST_APP}
WORKDIR /${RUST_APP}
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

# delete src and build files, triggers layer with compiled dependencies
RUN rm -r src
RUN rm target/release/deps/${RUST_APP}*

# copy program code and rebuild
ADD . .

RUN cargo build --release

###########
# Runtime #
###########
FROM debian:buster-slim
# use the global variable
ARG RUST_APP
# location of the program
ARG APP=/usr/src/rust_program

# openssl for rust mysql crate
RUN apt-get update && \
    apt-get install openssl -y && \ 
    rm -rf /var/lib/apt/lists/*

# create non-root user to run program
ENV APP_USER=rust_user
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /${RUST_APP}/target/release/${RUST_APP} ${APP}/program

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
CMD ["./program"]