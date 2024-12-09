FROM python:3.9.21-alpine3.20

RUN pip install Flask

WORKDIR /src
COPY . /src

CMD ["python", "server.py"]