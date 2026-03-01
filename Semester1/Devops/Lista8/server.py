import os
import uvicorn
from fastapi import FastAPI
from random import randint

app = FastAPI()

@app.get("/")
async def listen():
    if randint(0, 101) > 50:
        print("Wylaczam aplikacje")
        os._exit(1)
    
    return {"status": "ACK", "message": "Wszystko dziala poprawnie"}

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
