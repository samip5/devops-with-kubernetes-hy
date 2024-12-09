from flask import Flask
from os import environ

app = Flask(__name__)

if __name__ == '__main__':
    port = environ.get('PORT', 5000)
    app.run(port=port)