# Use the official Rust Docker image as the base image
FROM rust:bullseye

# Create a new directory to copy the project files into
RUN mkdir /app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the project dependencies
#RUN cargo build --release
#RUN cargo build 

# Copy the source code to the container
COPY src ./src

# Build the project
#RUN cargo build --release
RUN cargo build 

# Expose port 8080 for the web server
EXPOSE 8080

# Start the web server when the container starts
#CMD ["cargo", "run", "--release"]
CMD ["cargo", "run"]