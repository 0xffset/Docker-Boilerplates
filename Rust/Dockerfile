# variable of the project name (defined in Cargo.toml)
ARG RUST_APP=rust_app

################
# Build system #
################
FROM rust as builder
# use the global variable
ARG RUST_APP

# create project, copy dependencies and build with default src
RUN USER=root cargo new ${RUST_APP}
WORKDIR /${RUST_APP}
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# delete src, triggers layer with compiled dependencies
RUN rm src/*.rs

# copy program code, delete old binary and rebuild
ADD . ./
RUN rm ./target/release/${RUST_APP}
RUN cargo build --release

###########
# Runtime #
###########
FROM debian:buster-slim
# use the global variable
ARG RUST_APP
# location of the program
ARG APP=/usr/src/rust_program

RUN apt-get update \
    && rm -rf /var/lib/apt/lists/*

# create non-root user to run program
ENV APP_USER=rust_user
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /${RUST_APP}/target/release/${RUST_APP} ${APP}/program

# gives user permissions to run the program
RUN chown -R $APP_USER:$APP_USER ${APP}
# change from root user to user and run the program 
USER $APP_USER
WORKDIR ${APP}
CMD ["./program"]