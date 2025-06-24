FROM selenium/node-chrome:latest
WORKDIR /usr/bin/
COPY target/release/invisible_fun /usr/bin/
CMD ["invisible_fun"]
EXPOSE 1032