FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    imagemagick \
    unpaper \
    parallel \
    python3 \
    python3-pip \
    && pip3 install page-dewarp

WORKDIR /data
COPY procesuj.sh usun_numer_strony.py ./
RUN chmod +x procesuj.sh usun_numer_strony.py

CMD ["bash", "-c", "parallel -j 4 ./procesuj.sh {} ::: *.JPG"]
