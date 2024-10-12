# docker build -t devalanfernandes/fiap-techchallenge-fase2:latest .
# docker push devalanfernandes/fiap-techchallenge-fase2:latest

# Etapa 1: Build da aplicação
FROM rust:latest AS build

# Instala o CMake e ferramentas de build
RUN apt-get update && apt-get install -y \
    cmake \
    build-essential \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Cria um novo projeto shell com o nome em snake_case
RUN USER=root cargo new --bin fiap_tech_challenge_fase2
WORKDIR /fiap_tech_challenge_fase2

# Copia os arquivos de configuração do projeto
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Compila o projeto (apenas para cache das dependências)
RUN cargo build --release

# Remove o arquivo principal gerado anteriormente
RUN rm src/*.rs

# Copia o código fonte da aplicação
COPY ./src ./src

# Compila o projeto em modo release
RUN cargo build --release

# Etapa 2: Imagem final com GLIBC recente
FROM ubuntu:22.04 

# Instala as dependências de tempo de execução necessárias
RUN apt-get update && apt-get install -y \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

# Define o diretório de trabalho na imagem final
WORKDIR /usr/src/app

# Copia o binário da etapa de build
COPY --from=build /fiap_tech_challenge_fase2/target/release/fiap_tech_challenge_fase2 .

# Define a porta que será exposta
EXPOSE 8080
EXPOSE 443

# Define o comando de inicialização do contêiner
CMD ["./fiap_tech_challenge_fase2"]