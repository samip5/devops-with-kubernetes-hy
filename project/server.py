from flask import Flask
from os import environ

app = Flask(__name__)

@app.route("/")
def index():
    return "Hello world"

if __name__ == '__main__':
    port = environ.get('PORT', 5000)
    app.run(port=port)