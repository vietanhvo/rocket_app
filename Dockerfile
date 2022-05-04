# Build phase
FROM rust:1.58.1 as builder

RUN USER=root cargo new --bin rocket-app
WORKDIR ./rocket-app
# Build dependencies
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
# Build binary
ADD . ./

RUN rm ./target/release/deps/rocket_app*
RUN cargo build --release

# Run phase
FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libmariadb-dev-compat libmariadb-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser
# Create new user to run the binary
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rocket-app/target/release/main ${APP}/rocket-app
COPY --from=builder /rocket-app/Rocket.toml ${APP}/Rocket.toml

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./rocket-app"]
