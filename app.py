from flask import Flask, request, jsonify
import requests

app = Flask(__name__)

RUST_BACKEND_URL = "http://127.0.0.1:8000/markdown"

@app.route('/convert', methods=['POST'])
def convert_markdown():
    data = request.get_json()
    response = requests.post(RUST_BACKEND_URL, json=data['markdown'])
    return jsonify({"html": response.text})

if __name__ == "__main__":
    app.run(port=5000)
