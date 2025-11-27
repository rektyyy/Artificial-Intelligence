import sys
import uvicorn
from fastapi import FastAPI
from random import randint

app = FastAPI()

@app.get("/")
async def listen():
    if randint(0, 101) > 50:
        print("Wylaczam aplikacje")
        # sys.exit(1) kończy proces Pythona z kodem błędu 1 (oznaczającym błąd)
        sys.exit(1)
    
    return {"status": "ACK", "message": "Wszystko dziala poprawnie"}

if __name__ == "__main__":
    # Uruchomienie serwera bezpośrednio z pliku
    uvicorn.run(app, host="0.0.0.0", port=8000)