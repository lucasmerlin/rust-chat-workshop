FROM rust:1.63

WORKDIR /usr/src/app
COPY . .

RUN cd server && cargo install --path .

EXPOSE 6789

CMD ["server"]