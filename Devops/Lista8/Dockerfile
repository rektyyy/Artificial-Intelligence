FROM python:3.9-slim

WORKDIR /app

# Instalacja zależności
RUN pip install fastapi uvicorn

# Kopiowanie kodu
COPY server.py .

# Wystawienie portu
EXPOSE 8000

# Uruchomienie aplikacji
CMD ["python", "server.py"]