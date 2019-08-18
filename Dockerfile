# https://hub.docker.com/_/rust?tab=tags
FROM rust:1.37.0 as builder

# musl-gcc
RUN apt-get update \
	&& apt-get install -y \
		musl-dev \
		musl-tools \
		ca-certificates \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
# cache deps https://blog.jawg.io/docker-multi-stage-build/
RUN USER=root cargo init
COPY Cargo.toml .
RUN cargo build --target x86_64-unknown-linux-musl --release

# now copy the updated src and build artifacts
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl --release \
	&& trip /usr/src/app/target/x86_64-unknown-linux-musl/release/action-pagerduty

FROM scratch

# https://help.github.com/en/articles/metadata-syntax-for-github-actions#about-yaml-syntax-for-github-actions
LABEL version="0.1.0" \
  repository="https://github.com/meetup/actions-pagerduty/" \
  homepage="https://github.com/meetup/actions-pagerduty" \
  maintainer="Meetup" \
  "com.github.actions.name"="PagerDuty" \
  "com.github.actions.description"="Sends PagerDuty notifications as a result of Github Actions" \
  "com.github.actions.icon"="alert-triangle" \
  "com.github.actions.color"="green"

COPY --from=builder \
	/etc/ssl/certs/ca-certificates.crt \
	/etc/ssl/certs/
COPY --from=builder \
	/usr/src/app/target/x86_64-unknown-linux-musl/release/action-pagerduty \
	/action-pagerduty
ENTRYPOINT ["/action-pagerduty"]