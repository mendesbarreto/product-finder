ARG PROJECT_NAME=xbox-finder

FROM rust:1.54 as builder
ARG PROJECT_NAME

RUN USER=root cargo new --bin ${PROJECT_NAME}
WORKDIR ./${PROJECT_NAME}

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/xbox_finder*
RUN cargo build --release

FROM debian:buster-slim
ARG PROJECT_NAME
ARG APP=/usr/src/app
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/* \

EXPOSE 8080

ENV TZ=ETC/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

RUN echo "Created user $APP_USER"

COPY --from=builder /${PROJECT_NAME}/target/release/${PROJECT_NAME} ${APP}/${PROJECT_NAME}

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR $APP

CMD ["./xbox-finder"]