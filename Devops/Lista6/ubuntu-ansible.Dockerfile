FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y openssh-server python3 sudo && \
    mkdir /var/run/sshd

RUN useradd -m -s /bin/bash ansible && \
    echo 'ansible:ansible' | chpasswd && \
    usermod -aG sudo ansible && \
    echo 'ansible ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

RUN sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config && \
    sed -i 's/PasswordAuthentication no/PasswordAuthentication yes/' /etc/ssh/sshd_config && \
    mkdir -p /home/ansible/.ssh && \
    chmod 700 /home/ansible/.ssh

EXPOSE 22

CMD ["/usr/sbin/sshd", "-D"]