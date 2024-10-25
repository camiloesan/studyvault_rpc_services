# Use the official MySQL image from the Docker Hub
FROM mysql:latest

# Set environment variables
# These will be used to configure the MySQL server.
ENV MYSQL_ROOT_PASSWORD=123456
ENV MYSQL_DATABASE=study_vault
ENV MYSQL_USER=sv_user
ENV MYSQL_PASSWORD=123456

# Copy the SQL initialization script into the appropriate directory
# This directory is automatically processed by the MySQL Docker image at startup.
COPY db_script.sql /docker-entrypoint-initdb.d/

# Expose the MySQL default port
EXPOSE 3306

# Optionally, you can define a health check to ensure the MySQL server is running properly
HEALTHCHECK --interval=30s --timeout=10s --retries=5 \
    CMD mysqladmin ping -h localhost || exit 1
