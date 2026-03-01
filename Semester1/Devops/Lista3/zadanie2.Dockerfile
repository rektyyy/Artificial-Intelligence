FROM ubuntu:latest

RUN apt-get update
RUN apt-get install ssh iproute2 openssh-server -y

RUN mkdir -p /root/.ssh
RUN mkdir -p /var/run/sshd

COPY id_ed25519.pub /root/.ssh/authorized_keys
RUN chmod 600 /root/.ssh/authorized_keys
RUN echo 'PermitRootLogin yes' >> /etc/ssh/sshd_config

EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]
