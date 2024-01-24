FROM debian:bullseye

# Install commands
RUN apt-get update && apt-get install wget sudo curl gpg -y

# Get Azure functions core tools
RUN wget -q https://packages.microsoft.com/config/ubuntu/18.04/packages-microsoft-prod.deb -O packages-microsoft-prod.deb
RUN sudo dpkg -i packages-microsoft-prod.deb
RUN sudo apt-get update -y
RUN sudo apt-get install dotnet-sdk-2.1 -y
RUN sudo apt-get install azure-functions-core-tools-4 -y

# Get Rust
RUN sudo apt install curl build-essential gcc make -y
RUN apt-get update
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain 1.75.0

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Compile
RUN apt-get install jq pkg-config libssl-dev -y
COPY . /func
WORKDIR /func/scraper
RUN cargo build --release

# Run
EXPOSE 7071
EXPOSE 46075
RUN chmod +x run.sh
CMD ["sh", "/func/scraper/run.sh"]