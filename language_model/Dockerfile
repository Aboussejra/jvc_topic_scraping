FROM tiangolo/uvicorn-gunicorn:python3.9

COPY ./poetry.toml /poetry.toml

COPY ./poetry.lock /poetry.lock

COPY ./pyproject.toml /pyproject.toml

RUN set -ex; \
    # Installing poetry
    curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python3 - --version 1.1.13;\
    # Make poetry available system wide to update dependencies
    ln -s /root/.poetry/bin/poetry /usr/bin/poetry ;\
    poetry install

COPY ./app /app

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
