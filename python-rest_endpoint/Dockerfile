# Use the official Python 3.13 Alpine image from the Docker Hub
FROM python:3.13-alpine

# Set the working directory in the container
WORKDIR /app

# Copy the requirements file into the container
COPY requirements.txt .

# Install the dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the application code into the container
COPY . .

# Set the environment variables
ENV ONTAP_HOST=192.168.0.111
ENV ONTAP_USER=admin
ENV ONTAP_PASSWD=Netapp1!

# Specify the entrypoint command to run the application
ENTRYPOINT ["fastapi", "run", "api.py"]