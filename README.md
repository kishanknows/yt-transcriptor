# yt-transcriptor

A lightweight and efficient tool to extract and transcribe YouTube video subtitles using Rust.

## 🚀 Demo

Experience the live application here:

👉 [yt-transcriptor on Render](https://yt-transcriptor-aq4c.onrender.com)

## 🛠️ Features

* **Fast and Lightweight**: Built with Rust for optimal performance.
* **Subtitle Extraction**: Retrieves English subtitles from YouTube videos.
* **Web Interface**: User-friendly interface for easy interaction.
* **No Authentication Required**: Access the tool without any sign-in.

## 📦 Backend Installation

### Prerequisites

Ensure you have the following installed:

* [Rust](https://www.rust-lang.org/): The programming language used for development.
* [Docker](https://www.docker.com/): Optional, for containerized deployment.

### Steps

1. Navigate to the backend folder:

   ```bash
   cd yt-transcriptor/backend
   ```

2. Build and run the backend locally:

   ```bash
   cargo run
   ```

3. Access the backend API at `http://localhost:8000`.

### Docker Setup for Backend

1. Build the Docker image:

   ```bash
   docker build -t yt-transcriptor-backend ./backend
   ```

2. Run the Docker container:

   ```bash
   docker run -p 8000:8000 yt-transcriptor-backend
   ```

3. Open your browser or API client and visit `http://localhost:8000` to use the backend.

## 📝 License

This project is licensed under the MIT License.
