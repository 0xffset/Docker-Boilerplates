################
# Build system #
################
FROM rust as builder
# use the global variable
ARG RUST_APP
ARG DB_URL

RUN cargo install diesel_cli --no-default-features --features "mysql"

# create project, copy dependencies and build with default src
RUN USER=root cargo new ${RUST_APP}
WORKDIR /${RUST_APP}
COPY Cargo.toml Cargo.toml
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
ARG DB_URL
ARG APP

RUN apt-get update && \
    apt-get install libmariadb3 -y  && \
    rm -rf /var/lib/apt/lists/*

# create non-root user to run program
ENV APP_USER=rust_user
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# copy program
COPY --from=builder /${RUST_APP}/target/release/${RUST_APP} ${APP}/program

# copy diesel related stuff
COPY --from=builder /usr/local/cargo/bin/diesel ${APP}/diesel
COPY --from=builder /${RUST_APP}/Cargo.toml ${APP}/Cargo.toml

# copy wait-for-it.sh 
COPY --from=builder /${RUST_APP}/wait-for-it.sh ${APP}/wfi.sh

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}

# diesel db url env
ENV DATABASE_URL ${DB_URL}