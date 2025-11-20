FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y openssh-server python3 && \
    mkdir /var/run/sshd

RUN echo 'root:root' | chpasswd

RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config && \
    sed -i 's/PasswordAuthentication no/PasswordAuthentication yes/' /etc/ssh/sshd_config

RUN mkdir /tmp/solution/ &&\
    touch /tmp/solution/asd

EXPOSE 22

CMD ["/usr/sbin/sshd", "-D"]
