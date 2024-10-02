# Use the official PyTorch image as the base image
FROM pytorch/pytorch:2.4.1-cuda12.1-cudnn9-devel

# Set environment variable to prevent Python from writing .pyc files
ENV PYTHONDONTWRITEBYTECODE=1
# Set environment variable to buffer stdout/stderr for easier debugging
ENV PYTHONUNBUFFERED=1
ENV PATH=$PATH:/root/.local/bin

# Install system dependencies including PortAudio
RUN apt-get update && apt-get install -y \
    portaudio19-dev \
    && apt-get clean

# Create a directory for the application
WORKDIR /app

# Copy the requirements.txt file first to leverage Docker cache
COPY requirements.txt /app/

# Install Python dependencies
RUN pip install --upgrade pip \
    && pip install --no-cache-dir --user -r requirements.txt

# Copy the rest of the application code
COPY . /app/

# Define the default command to run the watchdog script that will watch for file changes
CMD watchmedo shell-command --patterns="*.py" --recursive --command='python app.py' --wait
