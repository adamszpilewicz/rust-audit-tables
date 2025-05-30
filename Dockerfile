FROM postgres:16

# Install wal2json build dependencies
RUN apt-get update && apt-get install -y \
    git \
    build-essential \
    postgresql-server-dev-16 \
    && git clone https://github.com/eulerto/wal2json \
    && cd wal2json \
    && make && make install \
    && cd .. && rm -rf wal2json \
    && apt-get remove --purge -y git build-essential postgresql-server-dev-16 \
    && apt-get autoremove -y && apt-get clean