FROM python:3.13.0-alpine3.20 AS builder

WORKDIR /app
COPY . /app

RUN python3 -m venv /app/venv
ENV PATH="/app/venv/:$PATH"

COPY  requirements.txt .
RUN venv/bin/pip install -r requirements.txt

FROM python:3.13.0-alpine3.20

COPY --from=builder /app /app

RUN useradd fastapi
RUN chown -R fastapi:fastapi /app
USER fastapi

# Setting env vars
ENV ONTAP_HOST $ONTAP_HOST
ENV ONTAP_USERNAME $ONTAP_USERNAME
ENV ONTAP_PASSWORD $ONTAP_PASSWORD

EXPOSE 8000

ENV PATH="/app/venv/bin/:$PATH"

CMD ["fastapi run app.py"]